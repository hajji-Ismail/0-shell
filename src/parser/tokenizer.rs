use std::io::{self, Write};

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
            '\\' => escape_next = true,

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

            ' ' | '\t' if !in_quotes => {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
            }

            _ => current.push(c),
        }
    }

    if !current.is_empty() {
        tokens.push(current);
    }

    if in_quotes {
        return Err("unterminated quotes".to_string());
    }

    Ok(tokens)
}
