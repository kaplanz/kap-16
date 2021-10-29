use std::collections::HashMap;
use std::vec::IntoIter;

use crate::line::{Line, Source};

#[derive(Clone, Debug, Default)]
pub struct Scope {
    pub source: Vec<Source>,
    pub symbols: HashMap<String, usize>,
}

impl Scope {
    pub fn new(lines: Vec<Line>) -> Self {
        let mut scope = Self::default();
        // Construct using an IntoIter
        scope.construct(&mut lines.into_iter());
        // Ensure we finished the file
        // TODO
        // Return the constructed scope
        scope
    }

    fn construct(&mut self, iter: &mut IntoIter<Line>) {
        // Construct from lines iterator
        while let Some(line) = iter.next() {
            match line
                .tokens
                .iter()
                .map(String::as_str)
                .collect::<Vec<&str>>()[..]
            {
                [".", "begin"] => {
                    let mut scope = Self::default();
                    scope.construct(iter);
                    self.source.push(Source::Scope(scope));
                }
                [".", "end"] => return,
                [symbol, ":"] => {
                    self.symbols.insert(symbol.to_string(), self.source.len());
                }
                _ => self.source.push(Source::Line(line)),
            }
        }
    }
}
