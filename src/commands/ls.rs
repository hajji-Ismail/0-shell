use crate::utils::Parsing;
use std::fs;

fn flag(flags: Vec<String>) -> Result<(bool, bool, bool), String> {
    let mut all = false;
    let mut long = false;
    let mut classify = false;
    for flag in flags {
        if flag.starts_with("--") {
            match flag.as_str() {
                "--long" => long = true,
                "--all" => all = true,
                "--classify" => classify = true,
                _ => return Err(format!("ls: unrecognized option {}", flag)),
            }
        } else {
            let sub_flag = flag.strip_prefix("-").unwrap();
            for c in sub_flag.chars() {
                match c {
                    'a' => all = true,
                    'l' => long = true,
                    'F' => classify = true,
                    _ => return Err(format!("ls: invalid option -- '{}'", sub_flag)),
                }
            }
        }
    }

    Ok((all, long, classify))
}
pub fn ls(tokens: Parsing) {
    let mut flag_tuple = Ok((false, false, false));
    if !tokens.flag.is_empty() {
        flag_tuple = flag(tokens.flag)
    }
    match flag_tuple {
        Ok(res) => {
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

                            if !name.starts_with('.') {
                                println!("{}", name);
                            } else if res.0 {
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
        Err(e) => {
            println!("{e}")
        }
    }
}
