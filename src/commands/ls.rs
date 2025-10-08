use std::fs;
use crate::utils::Parsing;

pub fn ls(parsing: Parsing) {
    if parsing.flag.is_empty() {
     
        let paths = if parsing.arg.is_empty() {
            vec![".".to_string()]
        } else {
            parsing.arg.clone()
        };

        for path in paths.iter() {
          
            if paths.len() > 1 {
                println!("{}:", path);
            }

            match fs::read_dir(path) {
                Ok(entries_iter) => {
              
                    let mut entries: Vec<_> = entries_iter
                        .filter_map(|e| e.ok()) 
                        .collect();

                    entries.sort_by_key(|e| e.file_name());

                    for entry in entries {
                        let name = entry.file_name().to_string_lossy().into_owned();
                      
                        if !name.starts_with('.') {
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
}
