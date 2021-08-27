use lazy_static::lazy_static;
use regex::Regex;

static COMMENT: &str = ";";

pub fn tokenize(lines: Vec<String>) -> Vec<Vec<String>> {
    lines.into_iter().filter_map(split).collect()
}

fn split(line: String) -> Option<Vec<String>> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\b").unwrap();
    }
    let tokens: Vec<_> = line
        .split_whitespace() // split at whitespace
        .flat_map(|s| RE.split(s).filter(|s| !s.is_empty()))
        .map(String::from) // convert from &str -> String
        .take_while(|s| !s.eq(COMMENT)) // strip comments
        .collect();
    match tokens.len() {
        0 => None,
        _ => Some(tokens),
    }
}
