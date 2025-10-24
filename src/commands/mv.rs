use crate::parser::Parsing;
use std::fs;
pub fn mv(input: Parsing) {
  
    if input.args.is_empty() || !input.flags.is_empty() {
        println!("mv: missing file operand");
        return;
    } else if input.args.len() < 2 {
        println!(
            "mv: missing destination file operand after {}",
            input.args[0]
        );
        return;
    } else {
        let destination = &input.args[input.args.len() - 1];

        for src in &input.args[..input.args.len() - 1] {
            match fs::rename(src, destination) {
                Ok(_) => continue,

                Err(e) => eprintln!("mv: {}: {}", src, e),
            }
        }
    }
}
