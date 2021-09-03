use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::mem;
use std::path::PathBuf;

use log::error;

use crate::line::Line;
use crate::{iarch, inst, lex, uarch, AssemblerError, WORDSIZE};

#[derive(Clone, Debug)]
pub struct Unit {
    src: PathBuf,
    lines: Vec<Line>,
    symbols: HashMap<String, usize>,
}

impl Unit {
    pub fn new(src: PathBuf) -> io::Result<Self> {
        // Read the input file
        let f = File::open(&src)?;
        // Construct lines from file
        let lines: Vec<String> = BufReader::new(f).lines().collect::<Result<_, _>>()?;
        let mut lines: Vec<Line> = lines
            .into_iter()
            .enumerate()
            .map(|(idx, line)| Line::new(idx, line))
            .filter(|line| !line.tokens.is_empty())
            .collect();
        // Extract symbols from lines
        let symbols = lex::extract(&mut lines);
        // Construct a source unit
        Ok(Unit {
            src,
            lines,
            symbols,
        })
    }

    pub fn concat(mut self, mut other: Unit) -> Option<Self> {
        // Ensure no duplicate symbols
        let set1: HashSet<String> = self.symbols.keys().cloned().collect();
        let set2: HashSet<String> = other.symbols.keys().cloned().collect();
        let duplicates: Vec<_> = set1.intersection(&set2).collect();
        if !duplicates.is_empty() {
            error!(
                "Duplicate symbol(s) from `{}`: {:?}",
                other.src.display(),
                duplicates
            );
            return None;
        }
        // Increment indices in other unit's symbols
        other
            .symbols
            .iter_mut()
            .for_each(|(_, idx)| *idx += self.lines.len());
        // Concatonate translation units
        self.lines.extend(other.lines);
        self.symbols.extend(other.symbols);
        // Return combined unit
        Some(self)
    }

    pub fn relocate(&mut self) {
        // Substitute symbols with addresses
        let symbols = &self.symbols;
        let lines = &mut self.lines;
        lines.iter_mut().enumerate().for_each(|(idx, line)| {
            line.tokens
                .iter_mut()
                .skip(1)
                .filter(|token| symbols.contains_key(*token))
                .for_each(|token| {
                    let delta = (symbols[token] as iarch) - (idx as iarch + 1);
                    let delta = (WORDSIZE as iarch) * (delta as iarch);
                    mem::swap(token, &mut format!("{:#x}", delta))
                })
        });
    }

    pub fn assemble(&self) -> Result<Vec<uarch>, AssemblerError> {
        self.lines
            .iter()
            .map(|line| {
                inst::assemble(&line.tokens).map_err(|err| AssemblerError {
                    err,
                    loc: (self.src.clone(), line.number),
                    line: line.content.clone(),
                })
            })
            .collect()
    }
}
