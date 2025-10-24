use crate::parser::{tokenizer, expander};

#[derive(Debug)]
pub struct Parsing {
    pub command: String,
    pub args: Vec<String>,
    pub flags: Vec<String>,
}

pub fn parse(input: &str) -> Result<Parsing, String> {
    let tokens = tokenizer::tokenize(input)?;

    if tokens.is_empty() {
        return Err("empty command".to_string());
    }

    let mut args = Vec::new();
    let mut flags = Vec::new();

    let command = tokens[0].clone();

    for token in &tokens[1..] {
        if token.starts_with('-') && !token.starts_with("--") {
            flags.push(token.clone());
        } else {
            let expanded = expander::expand_env_vars(token, token.starts_with('\'') && token.ends_with('\''));
            args.push(expanded);
        }
    }

    Ok(Parsing { command, args, flags })
}
