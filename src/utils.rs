use std::{ io::{self, Write} };
#[derive(Debug)]
pub struct Parsing {
    pub command: String,
    pub arg: Vec<String>,
    pub flag: Vec<String>,
}
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
                if input !="" {

                    let parsing_res = parser(input);
                    match parsing_res {
                        Ok(res) => match res .command.as_str() {
                            _=> println!("not implemented yet")
                            
                        }, 
                        Err(err)=> {
                            println!("{err}");
                            continue;
                        }
                    }
                } 

            }, 
             Err(e) => {
                eprintln!("Error reading input: {}", e);
                break 'main;
            }

           
        }
    }
}
fn parser(input : &str) ->Result<Parsing, String> {
    let tokens : Vec<&str>=  input.split_whitespace().collect();
    let command = if !tokens.is_empty() {
        tokens[0].to_string()
    } else {
        "".to_string()  // Or handle empty input differently
    };

    let mut args : Vec<String> = vec![];
    let mut flags : Vec<String> = vec![];
        for token in &tokens[1..] {
        if token.starts_with('-') {
            flags.push(token.to_string());
        } else {
            args.push(token.to_string());
        }
    }
match command.as_str() {
    "echo" | "cd" | "ls" | "pwd" | "cat" | "cp" | "rm" | "mv" | "mkdir" =>
        Ok(Parsing { command, arg: args, flag: flags }),

    _ => Err(format!("Command {} not found", command)),
}



}
