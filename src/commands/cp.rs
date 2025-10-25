use std::fs;
use std::path::Path;

use crate::parser::Parsing;
pub fn cp(input: Parsing) {
    if input.args.is_empty() {
        println!("cp: missing file operand");
        return;
    }  
    if !input.flags.is_empty() {
        println!("cp: invalid option -- '{}'", input.flags[0])
    }

    if input.args.len() < 2 {
        println!(
            "cp: missing destination file operand after {}",
            input.args[0]
        );
        return;
    } else {
        let destination = &input.args[input.args.len() - 1];
        let is_dir = Path::new(destination).is_dir();
        for src in &input.args[..input.args.len() - 1] {
            if src == destination {
                println!("cp: '{}' and '{}' are the same file", src,src);
                continue
            }
            let src_path = Path::new(src);
            let dest_path = if is_dir{
                Path::new(destination).join(src_path.file_name().unwrap())
            } else {
                Path::new(destination).to_path_buf()
            };

            match fs::copy(src_path, &dest_path) {
                Ok(_) => {}
                Err(err) => eprintln!("cp:'{}': {}", src, err),
            }
        }
    }
}
