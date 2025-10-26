use std::env;
use std::fs;
use std::path::PathBuf;

use crate::parser::Parsing;

pub fn pwd(data: Parsing){
  if !data.flags.is_empty() {
        eprintln!(" pwd: {:?}: invalid option", data.flags);
        return;
    }
 match env::current_dir() {
        Ok(path) => {
            // Normal case — directory still exists
            println!("{}", path.display());
        }
        Err(_) => {
            // Fallback: read /proc/self/cwd (works on Linux)
            let proc_path = PathBuf::from("/proc/self/cwd");
            match fs::read_link(&proc_path) {
                Ok(link_path) => {
                    println!("{}", link_path.display());
                }
                Err(e2) => {
                    eprintln!("pwd: cannot access current directory: {}", e2);
                }
            }
        }
    }
}