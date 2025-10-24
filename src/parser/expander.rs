use std::env;

pub fn expand_env_vars(input: &str, in_single_quotes: bool) -> String {
    if in_single_quotes {
        // Single quotes → no expansion
        return input.to_string();
    }

    let mut result = String::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '$' {
            if let Some(&'{') = chars.peek() {
                chars.next(); // skip '{'
                let mut var_name = String::new();
                while let Some(&next) = chars.peek() {
                    if next == '}' {
                        chars.next();
                        break;
                    }
                    var_name.push(next);
                    chars.next();
                }

                if let Ok(val) = env::var(&var_name) {
                    result.push_str(&val);
                }
            } else {
                let mut var_name = String::new();
                while let Some(&next) = chars.peek() {
                    if next.is_alphanumeric() || next == '_' {
                        var_name.push(next);
                        chars.next();
                    } else {
                        break;
                    }
                }

                if let Ok(val) = env::var(&var_name) {
                    result.push_str(&val);
                }
            }
        } else {
            result.push(c);
        }
    }

    result
}
