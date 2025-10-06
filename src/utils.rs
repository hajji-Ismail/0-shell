use std::io;
 use std::io::Write;

pub fn input_loop() {
    'main: loop {
        print!("$");
        io::stdout().flush().unwrap();
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).unwrap();
        let input = user_input.trim();
        if input == "exit" {
            break 'main;
        }

    }
}
