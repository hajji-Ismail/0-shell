use std::{ env, io::{ self, Write } };

pub fn tokenize(input: &str) -> Result<Vec<(String, bool)>, String> {
    let mut full_input = String::from(input);

    loop {
        let trimmed_input = full_input.trim_end();
        let trailing_backslash = trimmed_input.ends_with('\\') && !trimmed_input.ends_with("\\\\");

        match internal_tokenize(&full_input) {
            Ok(tokens) => {
                if trailing_backslash {
                    full_input.pop(); // Remove the trailing backslash
                    print!("> ");
                    io::stdout().flush().unwrap();

                    let mut user_input = String::new();
                    match io::stdin().read_line(&mut user_input) {
                        Ok(_) => {
                            if user_input.is_empty() {
                                // Ctrl+D detected
                                return Err("".to_string());
                            }
                            full_input.push_str(&user_input.trim_end());
                        }
                        Err(_) => {
                            return Err("Failed to read input".to_string());
                        }
                    }
                } else {
                    return Ok(tokens);
                }
            }
            Err(quote_char) => {
                if quote_char == '\0' {
                    return Err("0-shell: Bad substitution".to_string());
                }
                full_input.push('\n');

                print!("{} ", if quote_char == '"' { "dquote>" } else { "quote>" });
                io::stdout().flush().unwrap();

                let mut user_input = String::new();
                match io::stdin().read_line(&mut user_input) {
                    Ok(_) => {
                        if user_input.is_empty() {
                            // Ctrl+D detected
                            return Err("".to_string());
                        }

                        full_input.push_str(&user_input.trim_end());
                    }
                    Err(_) => {
                        return Err("Failed to read input".to_string());
                    }
                }
            }
        }
    }
}

fn internal_tokenize(input: &str) -> Result<Vec<(String, bool)>, char> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut quote_char = '\0';
    let mut escape_next = false;
    let mut in_single_quotes = false;

    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if escape_next {
            current.push(c);
            escape_next = false;
            continue;
        }

        match c {
            '\\' => {
                if in_quotes && quote_char == '"' {
                    if let Some(&next_char) = chars.peek() {
                        match next_char {
                            '"' | '\\' | '$' | '`' => {
                                current.push(chars.next().unwrap());
                            }
                            _ => {
                                current.push('\\');
                                current.push(chars.next().unwrap());
                            }
                        }
                    } else {
                        current.push('\\');
                    }
                } else if in_quotes && quote_char == '\'' {
                    current.push('\\');
                } else {
                    escape_next = true;
                }
            }

            '"' | '\'' => {
                if in_quotes && c == quote_char {
                    if quote_char == '\'' {
                        in_single_quotes = false;
                    }
                    in_quotes = false;
                } else if !in_quotes {
                    in_quotes = true;
                    quote_char = c;
                    if c == '\'' {
                        in_single_quotes = true;
                    }
                } else {
                    current.push(c);
                }
            }

            ' ' | '\t' if !in_quotes => {
                if !current.is_empty() {
                    tokens.push((current.clone(), in_single_quotes));
                    current.clear();
                    in_single_quotes = false;
                }
            }

            '~' if !in_quotes && current.is_empty() => {
                match chars.peek() {
                    Some(&c) if c == '/' || c.is_whitespace() => {
                        current.push_str(&env::var("HOME").unwrap_or("/".to_string()));
                    }
                    None => {
                        current.push_str(&env::var("HOME").unwrap_or("/".to_string()));
                    }
                    Some(_) => current.push('~'),
                }
            }

            '$' if !in_single_quotes => {
                let mut var_name = String::new();
                let mut is_braced = false;

                if let Some(&next_char) = chars.peek() {
                    if next_char == '{' {
                        is_braced = true;
                        chars.next();
                    }
                }

                while let Some(&next_char) = chars.peek() {
                    if is_braced && next_char == '}' {
                        chars.next();
                        if var_name.is_empty() {
                            return Err('\0'); // Trigger bad substitution error
                        }
                        break;
                    } else if !is_braced && !next_char.is_alphanumeric() && next_char != '_' {
                        break;
                    }
                    var_name.push(chars.next().unwrap());
                }

                if !var_name.is_empty() {
                    if let Ok(value) = env::var(&var_name) {
                        current.push_str(&value);
                    }
                } else {
                    current.push('$');
                }
            }

            _ => current.push(c),
        }
    }

    if !current.is_empty() {
        tokens.push((current.clone(), in_single_quotes));
    }

    if in_quotes {
        Err(quote_char)
    } else {
        Ok(tokens)
    }
}
