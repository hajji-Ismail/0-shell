use std::fs;

use crate::utils::Parsing;
pub fn mkdir(input : Parsing){
    if !input.flag.is_empty() {
        println!("mkdir: invalid option -- '{}'", input.flag[0]);
        return 

    }
    if input.arg.is_empty() {
          println!("mkdir: missing operand");
        return 
    }
    for arg in input.arg {
        match fs::create_dir(arg) {
            Ok(_) => {
                continue;
            },
            Err(err)=> {
                println!("mkdir: cannot create directory {err}");
                continue;
            }
        }
    }
  

}