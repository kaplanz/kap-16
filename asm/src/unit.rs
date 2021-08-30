use std::collections::{HashMap, HashSet};
use std::mem;
use std::path::PathBuf;

use log::error;

use crate::inst::ParseInstructionError;
use crate::{iarch, inst, lex, uarch, WORDSIZE};

#[derive(Clone, Debug)]
pub struct Unit {
    path: PathBuf,
    symbols: HashMap<String, usize>,
    source: Vec<Vec<String>>,
}

impl Unit {
    pub fn new(path: PathBuf, mut source: Vec<Vec<String>>) -> Self {
        Unit {
            path,
            symbols: lex::extract(&mut source),
            source,
        }
    }

    pub fn concat(mut self, mut other: Unit) -> Option<Self> {
        // Ensure no duplicate symbols
        let set1: HashSet<String> = self.symbols.keys().cloned().collect();
        let set2: HashSet<String> = other.symbols.keys().cloned().collect();
        let duplicates: Vec<_> = set1.intersection(&set2).collect();
        if !duplicates.is_empty() {
            error!(
                "Duplicate symbol(s) from `{}`: {:?}",
                other.path.display(),
                duplicates
            );
            return None;
        }
        // Increment indices in other unit's symbols
        other
            .symbols
            .iter_mut()
            .for_each(|(_, idx)| *idx += self.source.len());
        // Concatonate translation units
        self.source.extend(other.source);
        self.symbols.extend(other.symbols);
        // Return combined unit
        Some(self)
    }

    pub fn subst(&mut self) {
        // Substitute symbols with addresses
        let symbols = self.symbols.clone();
        self.source.iter_mut().enumerate().for_each(|(idx, line)| {
            line.iter_mut()
                .skip(1)
                .filter(|token| symbols.contains_key(*token))
                .for_each(|token| {
                    let delta = (symbols[token] as iarch) - (idx as iarch + 1);
                    let delta = (WORDSIZE as iarch) * (delta as iarch);
                    mem::swap(token, &mut format!("{:#x}", delta))
                })
        });
    }

    pub fn parse(&self) -> Result<Vec<uarch>, ParseInstructionError> {
        self.source.iter().map(inst::parse).collect()
    }
}
