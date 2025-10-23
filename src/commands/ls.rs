use crate::utils::Parsing;
use chrono::{Local, TimeZone};
use std::fs;
use std::os::unix::fs::FileTypeExt;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::time::{SystemTime, UNIX_EPOCH};
 use nix::libc;
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
    let flag_tuple = if !tokens.flag.is_empty() {
        flag(tokens.flag)
    } else {
        Ok((false, false, false))
    };

    match flag_tuple {
        Ok((all, long, classify)) => {
            let paths = if tokens.arg.is_empty() {
                vec![".".to_string()]
            } else {
                tokens.arg.clone()
            };

            for path in paths.iter() {
                match fs::metadata(path) {
                    Ok(metadata) => {
                        if metadata.is_dir() {
                            if paths.len() > 1 {
                                println!("{}:", path);
                            }

                            if long {
                                total(vec![path.clone()], all);

                                if all {
                                    if let Ok(meta_current) = fs::metadata(path) {
                                        print_long(&meta_current, ".");
                                    }
                                    let parent_path = format!("{}/..", path);
                                    if let Ok(meta_parent) = fs::metadata(&parent_path) {
                                        print_long(&meta_parent, "..");
                                    }
                                }
                            }

                            match fs::read_dir(path) {
                                Ok(entries_iter) => {
                                    let mut entries: Vec<_> = entries_iter.filter_map(|e| e.ok()).collect();
                                    entries.sort_by_key(|e| e.file_name());

                                    for entry in entries {
                                        let name = entry.file_name().to_string_lossy().into_owned();
                                        if name.starts_with('.') && !all {
                                            continue;
                                        }

                                        if let Ok(meta) = entry.metadata() {
                                            if long {
                                                print_long(&meta, &name);
                                            } else if classify {
                                                print_classified(&entry, &name);
                                            } else {
                                                println!("{}", name);
                                            }
                                        }
                                    }
                                }
                                Err(err) => eprintln!("ls: cannot access '{}': {}", path, err),
                            }

                            if paths.len() > 1 {
                                println!();
                            }
                        } else {
                            
                            if long {
                                print_long(&metadata, path);
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
fn print_long(metadata: &fs::Metadata, name: &str) {
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

    let size_or_dev = if metadata.file_type().is_block_device() || metadata.file_type().is_char_device() {
        let rdev = metadata.rdev();
        let major = libc::major(rdev) ;
        let minor =  libc::minor(rdev) ;
        format!("{}, {}", major, minor)
    } else {
        metadata.len().to_string()
    };

    println!(
        "{}{} {:>1} {} {} {:>8} {} {}",
        file_type,
        perms,
        metadata.nlink(),
        user,
        group,
        size_or_dev,
        datetime.format("%b %e %H:%M"),
        name
    );
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

    println!("{}{}", name, suffix);
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
