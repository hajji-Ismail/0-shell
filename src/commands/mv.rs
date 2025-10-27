use crate::parser::Parsing;
use std::{fs, path::Path};
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

        let is_dir = Path::new(destination).is_dir();

        if is_dir {
            for src in &input.args[..input.args.len() - 1] {
                if src == destination {
                    println!("cp: '{}' and '{}' are the same file", src, src);
                    continue;
                }
                let src_path = Path::new(src);
                let dest_path = if is_dir {
                    Path::new(destination).join(src_path.file_name().unwrap())
                } else {
                    Path::new(destination).to_path_buf()
                };

                match fs::copy(src_path, &dest_path) {
                    Ok(_) => {}
                    Err(err) => eprintln!("mv:'{}': {}", src, err),
                }
                if let Err(e) = fs::remove_file(&src_path) {
                    eprintln!("rm: cannot remove '{}': {}", src, e);
                }
            }
        } else {
            if input.args.len() > 2 {
                println!("mv: target '{destination}' is not a directory");
                return;
            }
            for src in &input.args[..input.args.len() - 1] {
                match fs::rename(src, destination) {
                    Ok(_) => continue,

                    Err(e) => eprintln!("mv: {}: {}", src, e),
                }
            }
        }
    }
}
