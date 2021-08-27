//! # Assembler
//!
//! `asm` is an assembler for the KAP-16 microprocessor.

use std::path::Path;
use std::{io, mem, process};

use log::error;

mod inst;
mod lex;
mod rdwr;
mod unit;
mod util;

use self::unit::Unit;

#[allow(non_camel_case_types)]
type iarch = i16;
#[allow(non_camel_case_types)]
type uarch = u16;

const WORDSIZE: usize = mem::size_of::<uarch>();

#[derive(Debug)]
pub struct Assembler {
    tus: Vec<Unit>,
}

impl Assembler {
    pub fn new() -> Self {
        Self {
            tus: Default::default(),
        }
    }

    pub fn source(&mut self, src: &Path) -> io::Result<()> {
        // Read the input file
        let lines = rdwr::read(src)?;
        // Tokenize lines
        let lines = lex::tokenize(lines);
        // Perform preprocessing
        // TODO
        // Create translation unit
        self.tus.push(Unit::new(src.to_path_buf(), lines));
        Ok(())
    }

    pub fn assemble(&mut self) {
        // Concatenate translation units
        let _unit = self
            .tus
            .clone()
            .into_iter()
            .reduce(|a, b| a.concat(b).unwrap_or_else(|| process::exit(1)))
            .unwrap_or_else(|| {
                error!("No sources to assemble!");
                process::exit(1)
            });
    }
}

impl Default for Assembler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {}
