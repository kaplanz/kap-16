use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

static COMMENT: &str = ";";
static SYMDELIM: &str = ":";

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

pub fn extract(source: &mut Vec<Vec<String>>) -> HashMap<String, usize> {
    // Extract symbols from source
    let mut idx = 0;
    let mut symbols = HashMap::new();
    source.retain(|line| {
        // Check if we have a symbol
        let is_symbol = line.len() == 2 && line[1] == SYMDELIM;
        if is_symbol {
            // Move the symbol, keeping track of the index
            symbols.insert(line[0].to_string(), idx);
        } else {
            idx += 1;
        }
        // Retain lines that aren't symbols
        !is_symbol
    });
    // Return extracted symbols
    symbols
}
