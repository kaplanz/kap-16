use crate::{lex, uarch};

#[derive(Clone, Debug)]
pub struct Line {
    pub number: usize,
    pub content: String,
    pub tokens: Vec<String>,
    pub word: Option<uarch>,
}

impl Line {
    pub fn new(number: usize, content: String) -> Self {
        let tokens = lex::tokenize(&content).unwrap_or_else(Vec::new); // tokenize line
        Line {
            number,
            content,
            tokens,
            word: None,
        }
    }
}
