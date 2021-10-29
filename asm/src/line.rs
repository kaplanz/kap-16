use crate::lex;
use crate::scope::Scope;

#[derive(Clone, Debug)]
pub enum Source {
    Line(Line),
    Scope(Scope),
}

#[derive(Clone, Debug)]
pub struct Line {
    pub number: usize,
    pub text: String,
    pub tokens: Vec<String>,
}

impl Line {
    pub fn new(number: usize, text: String) -> Self {
        let tokens = lex::tokenize(&text).unwrap_or_else(Vec::new);
        Self {
            number,
            text,
            tokens,
        }
    }
}
