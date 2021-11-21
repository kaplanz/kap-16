use std::collections::HashMap;
use std::mem;
use std::vec::IntoIter;

use crate::line::{Line, Source};
use crate::{WORDSIZE, iarch};

#[derive(Clone, Debug, Default)]
pub struct Scope {
    pub source: Vec<Source>,
    pub symbols: HashMap<String, usize>,
}

impl Scope {
    pub fn new(lines: Vec<Line>) -> Self {
        let mut scope = Self::default();
        // Construct using an IntoIter
        scope.ctor(&mut lines.into_iter());
        // Ensure we finished the file
        // TODO
        // Return the constructed scope
        scope
    }

    fn ctor(&mut self, iter: &mut IntoIter<Line>) {
        // Construct from lines iterator
        while let Some(line) = iter.next() {
            match line
                .tokens
                .iter()
                .map(String::as_str)
                .collect::<Vec<&str>>()[..]
            {
                [".", "begin" | "func"] => {
                    let mut scope = Self::default();
                    scope.ctor(iter);
                    self.source.push(Source::Scope(scope));
                }
                [".", "end"] => return,
                [symbol, ":"] => {
                    self.symbols
                        .insert(symbol.to_string(), self.source.len());
                }
                _ => self.source.push(Source::Line(line)),
            }
        }
    }

    pub fn subst(&mut self) {
        // Perform substitution in 2 passes (descending the scope tree):
        // 1. update symbol addresses
        self.update(&mut 0);
        // 2. replace symbol occurences
        self.replace(&mut 0, &HashMap::new());
    }

    pub fn flatten(self) -> Vec<Line> {
        self.source
            .into_iter()
            .flat_map(|src| match src {
                Source::Line(line) => vec![line],
                Source::Scope(scope) => scope.flatten(),
            })
            .collect()
    }

    fn update(&mut self, count: &mut usize) {
        self.symbols.iter_mut().for_each(|(_, v)| *v += *count);
        for src in self.source.iter_mut() {
            match src {
                Source::Line(_) => *count += 1,
                Source::Scope(scope) => {
                    scope.update(count);
                    for (_, v) in self.symbols.iter_mut() {
                        if *v > *count {
                            *v += *count;
                        }
                    }
                }
            }
        }
    }

    fn replace(&mut self, idx: &mut usize, symbols: &HashMap<String, usize>) {
        self.symbols.extend(symbols.clone());
        for src in self.source.iter_mut() {
            match src {
                Source::Line(line) => {
                    for token in line
                        .tokens
                        .iter_mut()
                        .filter(|t| self.symbols.contains_key(*t))
                    {
                        let symbol = self.symbols[token] as iarch;
                        let delta = symbol - (*idx as iarch + 1);
                        let delta = (WORDSIZE as iarch).saturating_mul(delta);
                        mem::swap(token, &mut format!("{:#x}", delta));
                    }
                    *idx += 1;
                }
                Source::Scope(scope) => scope.replace(idx, &self.symbols),
            }
        }
    }
}
