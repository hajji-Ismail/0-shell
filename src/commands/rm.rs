use std::fs;
use std::path::Path;

use crate::parser::Parsing;

pub fn rm(data: Parsing) {
    let recursive = if  data.flags.join("") == "-r".to_string() {
        true
    } else {
        false 
    } ;
        if !recursive && ! data.flags.is_empty() {
            println!("rm: invalid option -- {}", data.flags[0]);
            return
        }
    
    if data.args.is_empty() {
        eprintln!("rm: missing operand");
        return;
    }

    

    for arg in data.args {
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
