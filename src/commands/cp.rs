use std::fs;
use std::path::Path;

use crate::utils::Parsing;
pub fn cp(input: Parsing) {
    if !input.flag.is_empty() {
        println!("cp: invalid option -- '{}'", input.flag[0])
    }

    if input.arg.is_empty() {
        println!("cp: missing file operand");
        return;
    } else if input.arg.len() < 2 {
        println!(
            "cp: missing destination file operand after {}",
            input.arg[0]
        );
        return;
    } else {
        let destination = &input.arg[input.arg.len() - 1];
        if input.arg.len() > 2 {
            let dest_dir = Path::new(destination);
            if !dest_dir.exists() {
                println!("cp: target '{}' is not a directory", destination);
                return;
            }
            if !dest_dir.is_dir() {
                println!("cp: target '{}' is not a directory", destination);
                return;
            }
        }
        for src in &input.arg[..input.arg.len() - 1] {
            let src_path = Path::new(src);
            let dest_path = if Path::new(destination).is_dir() {
                Path::new(destination).join(src_path.file_name().unwrap())
            } else {
                Path::new(destination).to_path_buf()
            };

            match fs::copy(src_path, &dest_path) {
                Ok(_) => {}
                Err(err) => eprintln!("cp: cannot copy '{}': {}", src, err),
            }
        }
    }
}
