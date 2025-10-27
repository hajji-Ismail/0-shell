use crate::parser::Parsing;
use std::{fs, path::Path};

fn is_only_dots_and_slashes(s: &str) -> bool {
    s.chars().all(|c| c == '.' || c == '/')
}

pub fn mv(input: Parsing) {
    if input.args.is_empty() || !input.flags.is_empty() {
        eprintln!("mv: missing file operand");
        return;
    }

    if input.args.len() < 2 {
        eprintln!("mv: missing destination file operand after '{}'", input.args[0]);
        return;
    }

    let destination = &input.args[input.args.len() - 1];
    let dest_path = Path::new(destination);
    let dest_is_dir = dest_path.is_dir();

    for src in &input.args[..input.args.len() - 1] {
        if is_only_dots_and_slashes(src) {
            eprintln!("mv: invalid path '{}'", src);
            continue;
        }

        let src_path = Path::new(src);

        if src_path == dest_path {
            eprintln!("mv: '{}' and '{}' are the same file", src, destination);
            continue;
        }

        let target_path = if dest_is_dir {
            dest_path.join(src_path.file_name().unwrap_or_default())
        } else {
            dest_path.to_path_buf()
        };

   
        match fs::rename(src_path, &target_path) {
            Ok(_) => continue,
            Err(rename_err) => {
                // fallback: try copy + delete
                match fs::copy(src_path, &target_path) {
                    Ok(_) => {
                        if src_path.is_dir() {
                            if let Err(e) = fs::remove_dir_all(src_path) {
                                eprintln!("mv: cannot remove directory '{}': {}", src, e);
                            }
                        } else if let Err(e) = fs::remove_file(src_path) {
                            eprintln!("mv: cannot remove file '{}': {}", src, e);
                        }
                    }
                    Err(copy_err) => {
                        eprintln!("mv: cannot move '{}': {}", src, copy_err);
                        eprintln!("mv: rename failed: {}", rename_err);
                    }
                }
            }
        }
    }
}
