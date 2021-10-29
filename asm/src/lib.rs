//! # Assembler
//!
//! `asm` is an assembler for the KAP-16 microprocessor.

use std::error::Error;
use std::fmt::{self, Display};
use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::{mem, process};

use colored::Colorize;
use log::error;

mod inst;
mod lex;
mod line;
mod scope;
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
    units: Vec<Unit>,
    words: Vec<uarch>,
}

impl Assembler {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn source(&mut self, src: &Path) -> Result<(), Box<dyn Error>> {
        // Create a unit for this file
        let unit = Unit::new(src.to_path_buf())?;
        // Perform preprocessing
        // TODO
        // Ensure no duplicate symbols
        // TODO
        // Create translation unit
        self.units.push(unit);
        Ok(())
    }

    pub fn assemble(&mut self) -> Result<(), AssemblerError> {
        // Concatenate translation units
        let mut unit = self
            .units
            .clone()
            .into_iter()
            .reduce(|a, b| a.concat(b).unwrap_or_else(|| process::exit(1))) // XXX
            .unwrap_or_else(|| {
                error!("No sources to assemble!");
                process::exit(1);
            });
        // Replace symbols with their corresponding addresses
        unit.relocate();
        // Convert unit into binary
        self.words = unit.assemble()?;
        Ok(())
    }

    pub fn write(&self, out: &Path) -> io::Result<()> {
        // Write to the output file
        let mut f = File::create(out)?;
        f.write_all(
            &self
                .words
                .iter()
                .flat_map(|word| word.to_le_bytes())
                .collect::<Vec<_>>(),
        )
    }
}

#[derive(Debug)]
pub struct AssemblerError {
    err: Box<dyn Error>,
    loc: (PathBuf, usize),
    line: String,
}

impl Display for AssemblerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lineno = format!("{}", self.loc.1);
        writeln!(
            f,
            "{}{} {}",
            "error".red().bold(),
            ":".bold(),
            format!("{}", self.err).bold(),
        )?;
        writeln!(
            f,
            "{}{} {}:{}",
            " ".repeat(lineno.len()),
            "-->".blue().bold(),
            self.loc.0.display(),
            self.loc.1,
        )?;
        writeln!(f, "{} {}", " ".repeat(lineno.len()), "|".blue().bold())?;
        writeln!(f, "{} {}", format!("{} |", lineno).blue().bold(), self.line)?;
        write!(f, "{} {}", " ".repeat(lineno.len()), "|".blue().bold())
    }
}

impl Error for AssemblerError {}

#[cfg(test)]
mod tests {}
