//! # Assembler
//!
//! `asm` is an assembler for the KAP-16 microprocessor.

use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use std::{mem, process};

use log::error;

mod inst;
mod lex;
mod unit;
mod util;

use self::unit::Unit;

#[allow(non_camel_case_types)]
type iarch = i16;
#[allow(non_camel_case_types)]
type uarch = u16;

const WORDSIZE: usize = mem::size_of::<uarch>();

#[derive(Debug, Default)]
pub struct Assembler {
    state: State,
}

impl Assembler {
    pub fn new() -> Self {
        Self {
            state: State::Units(Default::default()),
        }
    }

    pub fn source(&mut self, src: &Path) -> Result<(), Box<dyn Error>> {
        if let State::Units(units) = &mut self.state {
            // Read the input file
            let f = File::open(src)?;
            let lines: Vec<String> = BufReader::new(f).lines().collect::<Result<_, _>>()?;
            // Tokenize lines
            let tokens: Vec<Vec<String>> = lines.into_iter().filter_map(lex::tokenize).collect();
            // Perform preprocessing
            // TODO
            // Ensure no duplicate symbols
            // TODO
            // Create translation unit
            units.push(Unit::new(src.to_path_buf(), tokens));
        } else {
            panic!("Invalid state!");
        }
        Ok(())
    }

    pub fn assemble(&mut self) -> Result<(), Box<dyn Error>> {
        if let State::Units(units) = &self.state {
            // Concatenate translation units
            let mut unit = units
                .clone()
                .into_iter()
                .reduce(|a, b| a.concat(b).unwrap_or_else(|| process::exit(1)))
                .unwrap_or_else(|| {
                    error!("No sources to assemble!");
                    process::exit(1);
                });
            // Replace symbols with their corresponding addresses
            unit.subst();
            // Convert unit into binary
            let words = unit.parse()?;
            // Update state
            self.state = State::Words(words);
            Ok(())
        } else {
            panic!("Invalid state!");
        }
    }

    pub fn write(&self, out: &Path) -> io::Result<()> {
        if let State::Words(words) = &self.state {
            // Write to the output file
            let mut f = File::create(out)?;
            f.write_all(
                &words
                    .iter()
                    .flat_map(|word| word.to_le_bytes())
                    .collect::<Vec<_>>(),
            )
        } else {
            panic!("Invalid state!");
        }
    }
}

#[derive(Debug)]
enum State {
    Units(Vec<Unit>),
    Words(Vec<uarch>),
}

impl Default for State {
    fn default() -> Self {
        Self::Units(Default::default())
    }
}

#[derive(Debug)]
pub struct AssemblerError {
    error: Box<dyn Error>,
    loc: (&'static Path, usize),
}

impl Display for AssemblerError {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        todo!()
    }
}

impl Error for AssemblerError {}

#[cfg(test)]
mod tests {}
