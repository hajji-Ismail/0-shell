use std::{ io::{self, Write} };

use crate::commands::{self, *};
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
                            "pwd" => pwd::pwd(),
                            "echo" => commands::echo::echo(res.arg),
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
    let tokens = tokenize(input);
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
fn tokenize(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut quote_char = '\0'; // stores which quote type we're inside

    for c in input.chars() {
        match c {
            '"' | '\'' => {
                if in_quotes && c == quote_char {
                    // closing the same type of quote
                    in_quotes = false;
                } else if !in_quotes {
                    // opening quote
                    in_quotes = true;
                    quote_char = c;
                } else {
                    // inside one type of quote, but different char (e.g., "it's fine")
                    current.push(c);
                }
            }
            ' ' if !in_quotes => {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
            }
            _ => {
                current.push(c);
            }
        }
    }

    if !current.is_empty() {
        tokens.push(current);
    }

    tokens
}
