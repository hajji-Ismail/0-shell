use std::env;

use crate::utils::Parsing;

pub fn pwd(data: Parsing){
  if !data.flag.is_empty() {
        eprintln!(" pwd: {:?}: invalid option", data.flag);
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