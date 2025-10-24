use crate::parser::tokenizer;

#[derive(Debug)]
pub struct Parsing {
    pub command: String,
    pub args: Vec<String>,
    pub flags: Vec<String>,
}

pub fn parse(input: &str) -> Result<Parsing, String> {
    let tokens_with_quotes = tokenizer::tokenize(input)?;

    if tokens_with_quotes.is_empty() {
        return Err("empty command".to_string());
    }

    let mut args = Vec::new();
    let mut flags = Vec::new();

    let (command, _) = &tokens_with_quotes[0];

    // Process remaining tokens
    for (token, _in_single_quotes) in tokens_with_quotes.iter().skip(1) {
        if token.starts_with('-') && !token.starts_with("--") && token != "-" {
            flags.push(token.clone());
        } else {
            args.push(token.clone());
        }
    }

    Ok(Parsing {
        command: command.clone(),
        args,
        flags,
    })
}