use std::fs;
use std::path::Path;

use crate::utils::Parsing;

pub fn rm(data: Parsing) {
    if data.arg.is_empty() {
        eprintln!("rm: missing operand");
        eprintln!("Try 'rm --help' for more information.");
        return;
    }

    let recursive = data.flag.contains(&"-r".to_string());

    for arg in data.arg {
        let path = Path::new(&arg);

        if !path.exists() {
            eprintln!("rm: cannot remove '{}': No such file or directory", arg);
            continue;
        }

        if path.is_dir() {
            if recursive {
                if let Err(e) = fs::remove_dir_all(&path) {
                    eprintln!("rm: cannot remove '{}': {}", arg, e);
                }
            } else {
                eprintln!("rm: cannot remove '{}': Is a directory", arg);
            }
        } else if let Err(e) = fs::remove_file(&path) {
            eprintln!("rm: cannot remove '{}': {}", arg, e);
        }
    }
}
