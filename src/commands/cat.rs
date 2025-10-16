use crate::Parsing;
use std::io::{self, Write};
use std::fs;

pub fn cat(input: Parsing) {
    if !input.flag.is_empty() {
        println!("cat: unrecognized option '{}'", input.flag[0]);

        return;
    }
    if input.arg.is_empty() {
        'cat: loop {
            io::stdout().flush().unwrap();

            let mut user_input = String::new();
            let bytes_read = io::stdin().read_line(&mut user_input);

            match bytes_read {
                Ok(0) => {
                    break 'cat;
                }
                Ok(_) => {
                    let input = user_input.trim();

                    println!("{input}");
                }
                Err(e) => {
                    eprintln!("Error reading input: {}", e);
                    break 'cat;
                }
            }
        }
    } else {
        for path in input.arg {
            match fs::read_to_string(path.clone()) {
                Ok(content) => println!("{}", content),
                Err(e) => eprintln!("cat: {}: {}", path, e),
            }
        }
    }
}
