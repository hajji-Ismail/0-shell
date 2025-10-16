use std::env;

use crate::utils::Parsing;

pub fn pwd(data: Parsing){
  if !data.flag.is_empty() {
        eprintln!("0-shel: pwd: {:?}: invalid option", data.flag);
        eprintln!("pwd: usage: pwd [-LP]");
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