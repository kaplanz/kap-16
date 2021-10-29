use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;

use log::error;

use crate::line::{Line, Source};
use crate::scope::Scope;
use crate::{inst, uarch, AssemblerError};

#[derive(Clone, Debug)]
pub struct Unit {
    src: PathBuf,
    global: Scope,
}

impl Unit {
    pub fn new(src: PathBuf) -> io::Result<Self> {
        // Open the input file
        let f = File::open(&src)?;
        // Read lines from file
        let lines: Vec<Line> = BufReader::new(f)
            .lines()
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .enumerate()
            .map(|(idx, line)| Line::new(idx, line))
            .filter(|line| !line.tokens.is_empty())
            .collect();
        // Parse global scope from file
        let global = Scope::new(lines);
        // Return this translation unit
        Ok(Self { src, global })
    }

    pub fn concat(mut self, mut other: Unit) -> Option<Self> {
        // Ensure no duplicate symbols
        let set1: HashSet<String> = self.global.symbols.keys().cloned().collect();
        let set2: HashSet<String> = other.global.symbols.keys().cloned().collect();
        let dups: Vec<_> = set1.intersection(&set2).collect();
        if !dups.is_empty() {
            error!(
                "Duplicate symbol(s) from `{}`: {:?}",
                other.src.display(),
                dups
            );
            return None;
        }
        // Increment indices in other unit's symbols
        other
            .global
            .symbols
            .iter_mut()
            .for_each(|(_, idx)| *idx += self.global.source.len());
        // Concatonate translation units
        self.global.source.extend(other.global.source);
        self.global.symbols.extend(other.global.symbols);
        // Return combined unit
        Some(self)
    }

    pub fn relocate(&mut self) -> u8 {
        todo!();
    }

    pub fn assemble(&self) -> Result<Vec<uarch>, AssemblerError> {
        self.global
            .source
            .iter()
            .map(|source| match source {
                Source::Line(line) => line,
                _ => panic!("Expected Source::Line(_)"), // FIXME
            })
            .map(|line| {
                inst::assemble(&line.tokens).map_err(|err| AssemblerError {
                    err,
                    loc: (self.src.clone(), line.number),
                    line: line.text.clone(),
                })
            })
            .collect()
    }
}
