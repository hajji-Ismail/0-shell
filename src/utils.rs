use std::io::{self, Write};

pub fn input_loop() {
    'main: loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut user_input = String::new();
        let bytes_read = io::stdin().read_line(&mut user_input);

        match bytes_read {
            Ok(0) => {

                break 'main;
            }
            Ok(_) => {
                let input = user_input.trim();

                if input == "exit" {
                    break 'main;
                }

                println!("You entered: {}", input);
            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break 'main;
            }
        }
    }
}
