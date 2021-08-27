//! # Assembler
//!
//! `asm` is an assembler for the KAP-16 microprocessor.

use std::mem;
use std::path::Path;

mod inst;
mod io;
mod lex;
mod util;

#[allow(non_camel_case_types)]
type iarch = i16;
#[allow(non_camel_case_types)]
type uarch = u16;

const WORDSIZE: usize = mem::size_of::<uarch>();

pub struct Assembler<'a> {
    src: &'a Path,
    out: &'a Path,
}

impl<'a> Assembler<'a> {
    pub fn new(src: &'a Path, out: &'a Path) -> Self {
        Self { src, out }
    }

    pub fn assemble(&self) {
        // Read the input file
        let lines = io::read(self.src).unwrap();
        // Perform tokenization
        let lines = lex::tokenize(lines);
    }
}

#[cfg(test)]
mod tests {}
