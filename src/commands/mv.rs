use crate::utils::Parsing;
use std::fs;
pub fn mv(input: Parsing) {
  
    if input.arg.is_empty() || !input.flag.is_empty() {
        println!("mv: missing file operand");
        return;
    } else if input.arg.len() < 2 {
        println!(
            "mv: missing destination file operand after {}",
            input.arg[0]
        );
        return;
    } else {
        let destination = &input.arg[input.arg.len() - 1];

        for src in &input.arg[..input.arg.len() - 1] {
            match fs::rename(src, destination) {
                Ok(_) => continue,

                Err(e) => eprintln!("mv: {}: {}", src, e),
            }
        }
    }
}
