use std::env;

use crate::parser::Parsing;

pub fn pwd(data: Parsing){
  if !data.flags.is_empty() {
        eprintln!(" pwd: {:?}: invalid option", data.flags);
        return;
    }
    match env::current_dir() {
        Ok(path) => {
            println!("{}",path.display())
        },
        Err(e)=>{
            eprintln!("{}",e)
        }
    }
}