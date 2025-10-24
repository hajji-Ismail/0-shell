use std::{ io::{ self, Write } };
use crate::commands::{ self, * };
use crate::parser::*;

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
                if input != "" {
                    let parsing_res = parse(input);
                    match parsing_res {
                        Ok(res) =>
                            match res.command.as_str() {
                                "pwd" => commands::pwd::pwd(res),
                                "echo" => commands::echo::echo(res.args),
                                "cd" => commands::cd::cd(Some(&res.args.join(""))),
                                "rm" => commands::rm::rm(res),
                                "ls" => commands::ls::ls(res),
                                "mkdir" => commands::mkdir::mkdir(res),
                                "cp" => cp::cp(res),
                                "cat" => cat::cat(res),
                                "mv" => mv::mv(res),

                                _ => external::run_external_command(&res),
                            }
                        Err(err) => {
                            println!("{err}");
                            continue;
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break 'main;
            }
        }
    }
}

