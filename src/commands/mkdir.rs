use std::fs;

use crate::parser::Parsing;
pub fn mkdir(input : Parsing){
    if !input.flags.is_empty() {
        println!("mkdir: invalid option -- '{}'", input.flags[0]);
        return 

    }
    if input.args.is_empty() {
          println!("mkdir: missing operand");
        return 
    }
    for arg in input.args {
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