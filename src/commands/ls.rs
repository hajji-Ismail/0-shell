use crate::utils::Parsing;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::time::{SystemTime, UNIX_EPOCH};
use std::os::unix::fs::FileTypeExt;

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
                if paths.len() > 1 {
                    println!("{}:", path);
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

                            let metadata = match entry.metadata() {
                                Ok(m) => m,
                                Err(_) => continue,
                            };

                            if long {
                                print_long(&metadata);
                            }

                            if classify {
                                print_classified(&entry, &name);
                            } else {
                                println!("{}", name);
                            }
                        }
                    }
                    Err(err) => {
                        eprintln!("ls: cannot access '{}': {}", path, err);
                    }
                }

                if paths.len() > 1 {
                    println!();
                }
            }
        }
        Err(e) => eprintln!("{e}"),
    }
}

fn print_long(metadata: &fs::Metadata) {
    let permissions = metadata.permissions().mode();
    let file_type = if metadata.is_dir() { 'd' } else { '-' };
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

    let size = metadata.len();
    let modified = metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);
    // fix the hard code 
    let seconds = modified
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    
    print!("{}{} {:>1} {:>1} ", file_type, perms, 1, size);

    use chrono::{ Local,  TimeZone};
    let datetime = Local.timestamp_opt(seconds as i64, 0).unwrap();
    print!("{} ", datetime.format("%b %e %H:%M"));
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
