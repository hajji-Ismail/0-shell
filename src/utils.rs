use std::{ io::{ self, Write } };
use crate::commands::{ self, * };
use crate::parser::*;

pub fn input_loop() {
    'main: loop {
        print_path();
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
                                "cd" => commands::cd::cd(res),
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
fn print_path() {
    const BOLD: &str = "\x1b[1m";
    const RESET: &str = "\x1b[0m";
    const BLUE: &str = "\x1b[34m";

    match std::env::var("PWD") {
        Ok(pwd) => {
            print!("{}{}{}{}$", BOLD, BLUE, pwd, RESET);
        }
        Err(_) => {
            match std::env::current_dir() {
                Ok(path) => {
                    print!("{}{}{}{}$", BOLD, BLUE, path.display(), RESET);
                }
                Err(_) => {
                    print!("{}$", BOLD);
                }
            }
        }
    }
}
