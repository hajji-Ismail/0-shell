use crate::parser::Parsing;
use chrono::{Local, TimeZone};
use nix::libc;
use std::fs;
use std::os::unix::fs::{FileTypeExt, MetadataExt, PermissionsExt};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use users::{get_group_by_gid, get_user_by_uid};

fn flag(flags: Vec<String>) -> Result<(bool, bool, bool), String> {
    let mut all = false;
    let mut long = false;
    let mut classify = false;

    for flag in flags {
        if flag.starts_with("--") {
            match flag.as_str() {
                "--all" => all = true,
                "--classify" => classify = true,
                _ => return Err(format!("ls: unrecognized option '{}'", flag)),
            }
        } else if let Some(sub_flag) = flag.strip_prefix('-') {
            for c in sub_flag.chars() {
                match c {
                    'a' => all = true,
                    'l' => long = true,
                    'F' => classify = true,
                    _ => return Err(format!("ls: invalid option -- '{}'", c)),
                }
            }
        }
    }

    Ok((all, long, classify))
}

pub fn ls(tokens: Parsing) {
    let flag_tuple = if !tokens.flags.is_empty() {
        flag(tokens.flags)
    } else {
        Ok((false, false, false))
    };

    match flag_tuple {
        Ok((all, long, classify)) => {
            let paths = if tokens.args.is_empty() {
                vec![".".to_string()]
            } else {
                tokens.args.clone()
            };

            for (i, path) in paths.iter().enumerate() {
                match fs::metadata(path) {
                    Ok(metadata) => {
                        let mode = metadata.permissions().mode();
                    

                        if metadata.is_dir() {
                             if mode & 0o100 == 0 ||  mode & 0o400 == 0{
                        println!("ls: cannot access '{path}'': Permission denied");
                        continue;

                      }
                            if paths.len() > 1 {
                                println!("{}:", path);
                            }

                            if long {
                                total(vec![path.clone()], all);
                            }

                            // Print . and .. if -a
                            if all {
                                if long {
                                    if let Ok(meta_current) = fs::metadata(path) {
                                        print_long(&meta_current, ".", PathBuf::from(path), classify);

                                    }
                                    let parent_path = format!("{}/..", path);
                                    if let Ok(meta_parent) = fs::metadata(&parent_path) {
                                         print_long(&meta_parent, "..", PathBuf::from(&parent_path), classify);
                                    }
                                } else {
                                    if classify {
                                        println!("./");
                                        println!("../");
                                    } else {
                                        println!(".");
                                        println!("..");
                                    }
                                }
                            }

                            match fs::read_dir(path) {
                                Ok(entries_iter) => {
                                    let mut entries: Vec<_> =
                                        entries_iter.filter_map(|e| e.ok()).collect();
                                    entries.sort_by_key(|e| e.file_name().to_ascii_lowercase());

                                    for entry in entries {
                                        let name = entry.file_name().to_string_lossy().into_owned();
                                        if name.starts_with('.') && !all {
                                            continue;
                                        }

                                        if let Ok(meta) = entry.metadata() {
                                            if long {
                                                print_long(&meta, &name,entry.path() ,classify);
                                            } else if classify {
                                                print_classified(&entry, &name);
                                            } else {
                                                if needs_quotes(&name) {
                                                    println!("'{}'", name);
                                                } else {
                                                    println!("{}", name);
                                                }
                                            }
                                        }
                                    }
                                }
                                Err(err) => eprintln!("ls: cannot access '{}': {}", path, err),
                            }

                            if paths.len() > 1 && i < paths.len() - 1 {
                                println!();
                            }
                        } else {
                            if long {
                                print_long(&metadata, path, PathBuf::from(path),classify);
                            } else if classify {
                                let ft = metadata.file_type();
                                let suffix = if ft.is_dir() {
                                    "/"
                                } else if ft.is_symlink() {
                                    "@"
                                } else if ft.is_socket() {
                                    "="
                                } else if ft.is_fifo() {
                                    "|"
                                } else {
                                    ""
                                };
                                println!("{}{}", path, suffix);
                            } else {
                                println!("{}", path);
                            }
                        }
                    }
                    Err(err) => eprintln!("ls: cannot access '{}': {}", path, err),
                }
            }
        }
        Err(e) => eprintln!("{e}"),
    }
}

fn print_long(metadata: &fs::Metadata, name: &str,full_path : PathBuf, classify: bool) {
    let file_type = {
        let ft = metadata.file_type();
        if ft.is_dir() {
            'd'
        } else if ft.is_symlink() {
            'l'
        } else if ft.is_char_device() {
            'c'
        } else if ft.is_block_device() {
            'b'
        } else if ft.is_socket() {
            's'
        } else if ft.is_fifo() {
            'p'
        } else {
            '-'
        }
    };

    let permissions = metadata.permissions().mode();
    let perms = format!(
        "{}{}{}{}{}{}{}{}{}",
        if permissions & 0o400 != 0 { 'r' } else { '-' },
        if permissions & 0o200 != 0 { 'w' } else { '-' },
        if permissions & 0o100 != 0 { 'x' } else { '-' },
        if permissions & 0o040 != 0 { 'r' } else { '-' },
        if permissions & 0o020 != 0 { 'w' } else { '-' },
        if permissions & 0o010 != 0 { 'x' } else { '-' },
        if permissions & 0o004 != 0 { 'r' } else { '-' },
        if permissions & 0o002 != 0 { 'w' } else { '-' },
        if permissions & 0o001 != 0 { 'x' } else { '-' },
    );

    let uid = metadata.uid();
    let gid = metadata.gid();

    let user = get_user_by_uid(uid)
        .and_then(|u| u.name().to_str().map(|s| s.to_string()))
        .unwrap_or(uid.to_string());

    let group = get_group_by_gid(gid)
        .and_then(|g| g.name().to_str().map(|s| s.to_string()))
        .unwrap_or(gid.to_string());

    let modified = metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);
    let seconds = modified
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let datetime = Local.timestamp_opt(seconds as i64, 0).unwrap();

    let size_or_dev =
        if metadata.file_type().is_block_device() || metadata.file_type().is_char_device() {
            let rdev = metadata.rdev();
            let major = libc::major(rdev);
            let minor = libc::minor(rdev);
            format!("{}, {}", major, minor)
        } else {
            metadata.len().to_string()
        };

    // Determine classify suffix
    let suffix = if classify {
        if metadata.file_type().is_dir() {
            "/"
        } else if metadata.file_type().is_symlink() {
            "@"
        } else if metadata.file_type().is_socket() {
            "="
        } else if metadata.file_type().is_fifo() {
            "|"
        } else {
            ""
        }
    } else {
        ""
    };
    if needs_quotes(name) {
        print!(
            "{}{} {:>2} {:<8} {:<8} {:>8} {} '{}'{}",
            file_type,
            perms,
            metadata.nlink(),
            user,
            group,
            size_or_dev,
            datetime.format("%b %e %H:%M"),
            name,
            suffix
        );
    } else {
        print!(
            "{}{} {:>2} {:<8} {:<8} {:>8} {} {}{}",
            file_type,
            perms,
            metadata.nlink(),
            user,
            group,
            size_or_dev,
            datetime.format("%b %e %H:%M"),
            name,
            suffix
        );
    }
    if metadata.file_type().is_symlink() {
     
 
        if let Ok(target) = fs::read_link(&full_path) {
            print!(" -> {}", target.display());
        } 
        
        
    }
    println!();
}

fn print_classified(entry: &fs::DirEntry, name: &str) {
    let ft = match entry.file_type() {
        Ok(t) => t,
        Err(_) => {
            println!("{}", name);
            return;
        }
    };

    let suffix = if ft.is_dir() {
        "/"
    } else if ft.is_symlink() {
        "@"
    } else if ft.is_socket() {
        "="
    } else if ft.is_fifo() {
        "|"
    } else {
        ""
    };
    if needs_quotes(name) {
        println!("'{}'{}", name, suffix);
    } else {
        println!("{}{}", name, suffix);
    }
}

fn total(paths: Vec<String>, all: bool) {
    let mut total_blocks = 0;

    for path in paths.iter() {
        if let Ok(entries_iter) = fs::read_dir(path) {
            let entries: Vec<_> = entries_iter.filter_map(|e| e.ok()).collect();

            if all {
                if let Ok(meta_current) = fs::metadata(path) {
                    total_blocks += meta_current.blocks();
                }
                let parent_path = format!("{}/..", path);
                if let Ok(meta_parent) = fs::metadata(&parent_path) {
                    total_blocks += meta_parent.blocks();
                }
            }

            for entry in entries {
                let name = entry.file_name().to_string_lossy().into_owned();
                if name.starts_with('.') && !all {
                    continue;
                }
                if let Ok(meta) = entry.metadata() {
                    total_blocks += meta.blocks();
                }
            }
        }
    }

    println!("total {}", total_blocks / 2);
}

fn needs_quotes(name: &str) -> bool {
    let s = name.to_string();
    s.chars()
        .any(|c| c.is_whitespace() || c.is_control() || c == '\'' || c == '"' || c == '\\')
}
