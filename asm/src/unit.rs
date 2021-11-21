use std::collections::HashSet;
use std::error::Error;
use std::fmt::{self, Display};
use std::path::PathBuf;

use crate::line::Line;
use crate::scope::Scope;
use crate::{inst, uarch, VerboseError};

#[derive(Clone, Debug, Default)]
pub struct Unit {
    path: PathBuf,
    global: Scope,
}

impl Unit {
    pub fn new(path: PathBuf, lines: Vec<Line>) -> Self {
        Self {
            path,
            global: Scope::new(lines),
        }
    }

    pub fn concat(mut self, mut other: Unit) -> Result<Self, UnitError> {
        // Ensure no duplicate symbols
        let set1: HashSet<String> = self.global.symbols.keys().cloned().collect();
        let set2: HashSet<String> = other.global.symbols.keys().cloned().collect();
        let dups: Vec<_> = set1.intersection(&set2).cloned().collect();
        if !dups.is_empty() {
            return Err(UnitError::DuplicateSymbols(dups));
        }
        // Increment indices in other unit's symbols
        other
            .global
            .symbols
            .iter_mut()
            .for_each(|(_, idx)| *idx += self.global.source.len());
        // Concatenate translation units
        self.global.source.extend(other.global.source);
        self.global.symbols.extend(other.global.symbols);
        // Return combined unit
        Ok(self)
    }

    pub fn asm(mut self) -> Result<Vec<uarch>, VerboseError> {
        // Perform symbol substitutions
        self.global.subst();
        // Flatten the global scope
        let lines = self.global.flatten();
        // Assemble instructions
        lines
            .into_iter()
            .map(|line| {
                inst::asm(&line.tokens).map_err(|err| VerboseError {
                    err: From::from(err),
                    loc: (self.path.clone(), line.number),
                    line: line.text.clone(),
                })
            })
            .collect()
    }
}

#[derive(Debug)]
pub enum UnitError {
    DuplicateSymbols(Vec<String>),
}

impl Display for UnitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::DuplicateSymbols(dups) => format!("Duplicate symbols: {:?}", dups),
            }
        )
    }
}

impl Error for UnitError {}
