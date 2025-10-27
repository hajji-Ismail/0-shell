use std::fs;
use std::path::Path;

use crate::parser::Parsing;

pub fn rm(data: Parsing) {

   match flag(data.flags) {
    Ok(recursive) => {
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

    } , 
    Err( err) => {
        println!("{err}");
    }

   }
    
   
}
fn flag(flags : Vec<String>) ->Result<bool, String> {
    let mut recursive = false ;
     for flag in flags {
     if let Some(sub_flag) = flag.strip_prefix('-') {
            for c in sub_flag.chars() {
                match c {
                    'r' => recursive = true,

                    _ => return Err(format!("rm: invalid option -- '{}'", c)),
                }
            }
        }
    }
    Ok(recursive)
}