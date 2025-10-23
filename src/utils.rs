use std::{ io::{ self, Write } };
use std::env;
use std::path::Path;
use crate::commands::{ self, * };
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
                if input != "" {
                    let parsing_res = parser(input);
                    match parsing_res {
                        Ok(res) =>
                            match res.command.as_str() {
                                "pwd" => commands::pwd::pwd(res),
                                "echo" => commands::echo::echo(res.arg),
                                "cd" => commands::cd::cd(Some(&res.arg.join(""))),
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
fn parser(input: &str) -> Result<Parsing, String> {
    let (tokens, err) = tokenize(input);
    if err {
        return Err("zsh: no such user or named directory:".to_string());
    }
    let command = if !tokens.is_empty() {
        tokens[0].to_string()
    } else {
        "".to_string() // Or handle empty input differently
    };

    let mut args: Vec<String> = vec![];
    let mut flags: Vec<String> = vec![];
    for token in &tokens[1..] {
        if token.starts_with('-') {
            flags.push(token.to_string());
        } else {
            args.push(token.to_string());
        }
    }

    Ok(Parsing { command, arg: args, flag: flags })
}
fn tokenize(input: &str) -> (Vec<String> , bool){
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut quote_char = '\0';

    let  mut telda  = false ;
    for c in input.chars() {
        match c {
            '"' | '\'' => {
                if in_quotes && c == quote_char {
                    in_quotes = false;
                } else if !in_quotes {
                    // opening quote
                    in_quotes = true;
                    quote_char = c;
                } else {
                    // inside one type of quote, but different char ("it's fine")
                    current.push(c);
                }
            }
            ' ' if !in_quotes && !current.is_empty() => {
                telda =  false ;
                tokens.push(current.clone());
                current.clear()
            }
            _ => {
                if !in_quotes && current.is_empty() && c == '~' {
                    telda =  true ;
                    let home = match env::var("HOME") {
                        Ok(home) => Path::new(&home).to_path_buf(),
                        Err(_) => Path::new("/").to_path_buf(),
                    };
                    if let Some(path_str) = home.to_str() {
                        current.push_str(path_str);
                    }
                } else  {
                    if telda && in_quotes && c != '/' {
                        current = "~".to_string() ;
                        telda = false
                    }else if telda &&c == '/' {
                        println!("ho");
                        telda =false

                    } else if telda && !in_quotes  && c != '/' {
                     return (tokens, true)
                    }

                    current.push(c);
                }

            }
        }
    }

    if !current.is_empty() {
        tokens.push(current.clone());
    }

    // didnt enter closing cotes  so prpmpte user to enter closing cotes
    if in_quotes {
        tokens.last_mut().expect("No token to modify").push('\n');
        loop {
            if quote_char == '"' {
                print!("dquote>");
            } else if quote_char == '\'' {
                print!("quote>");
            }

            io::stdout().flush().unwrap();

            let mut user_input = String::new();
            let _ = io::stdin().read_line(&mut user_input);

            if let Some(pos) = user_input.find(quote_char) {
                let (first_part, second_part) = user_input.split_at(pos);
                // println!("{}1 {}2 ", first_part, second_part);
                tokens.last_mut().expect("No token to modify").push_str(first_part);

                let last = if let Some(last_part) = second_part.strip_prefix(quote_char) {
                    last_part.to_string()
                } else {
                    second_part.to_string()
                };

                tokens.push(last);
                break;
            } else {
                tokens.last_mut().expect("No token to modify").push_str(&user_input);
            }
        }
        if let Some(token) = tokens.last_mut() {
            if token.ends_with('\n') {
                token.pop();
                if token.ends_with('\r') {
                    token.pop();
                }
            }
        }
    }

   ( tokens, false)
}
