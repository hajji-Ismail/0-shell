use std::{ env, io::{ self, Write } };

pub fn tokenize(input: &str) -> Result<Vec<String>, String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut quote_char = '\0';
    let mut escape_next = false;

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
                    if let Some(next_char) = chars.next() {
                        // Only special characters are escaped
                        match next_char {
                            '"' | '\\' | '$' | '`' => current.push(next_char),
                            _ => {
                                current.push('\\');
                                current.push(next_char);
                            }
                        }
                    } else {
                        current.push('\\');
                    }
                } else if in_quotes && quote_char == '\'' {
                    current.push('\\'); // literal
                } else {
                    // outside quotes: escape next char
                    if let Some(next_char) = chars.next() {
                        current.push(next_char);
                    }
                }
            }

            '"' | '\'' => {
                if in_quotes && c == quote_char {
                    in_quotes = false;
                } else if !in_quotes {
                    in_quotes = true;
                    quote_char = c;
                } else {
                    current.push(c);
                }
            }

            // Handle whitespace as token separators when not quoted
            ' ' | '\t' if !in_quotes => {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
            }

            // Handle tilde (~ → $HOME)
            '~' if !in_quotes && current.is_empty() => {
                let next = chars.peek();
                match next {
                    Some('/') | None => {
                        if let Ok(home) = env::var("HOME") {
                            current.push_str(&home);
                        } else {
                            current.push('/');
                        }
                    }
                    Some(_) => {
                        current.push('~');
                    }
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

    // Handle unterminated quotes (multi-line continuation)
    if in_quotes {
        tokens.last_mut().expect("No token to modify").push('\n');
        loop {
            if quote_char == '"' {
                print!("dquote>");
            } else {
                print!("quote>");
            }
            io::stdout().flush().unwrap();

            let mut user_input = String::new();
            if io::stdin().read_line(&mut user_input).is_err() {
                break;
            }

            if let Some(pos) = user_input.find(quote_char) {
                let (first_part, remainder) = user_input.split_at(pos);
                tokens.last_mut().unwrap().push_str(first_part);
                if let Some(after) = remainder.strip_prefix(quote_char) {
                    tokens.push(after.to_string());
                }
                break;
            } else {
                tokens.last_mut().unwrap().push_str(&user_input);
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

    println!("{:?} bbbb",tokens);

    Ok(tokens)
}
