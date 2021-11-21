//! # Assembler
//!
//! `asm` is an assembler for the KAP-16 microprocessor.

use std::error::Error;
use std::fmt::{self, Display};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::mem;
use std::path::{Path, PathBuf};

use colored::Colorize;
use line::Line;

mod inst;
mod lex;
mod line;
mod prep;
mod scope;
mod unit;
mod util;

use crate::unit::Unit;

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

    pub fn src(&mut self, path: &Path) -> Result<(), Box<dyn Error>> {
        // Open the input file
        let f = File::open(&path)?;
        // Read lines from file
        let mut lines: Vec<Line> = BufReader::new(f)
            .lines()
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .enumerate()
            .map(|(idx, line)| Line::new(idx, line))
            .filter(|line| !line.tokens.is_empty())
            .collect();
        // Perform preprocessing
        prep::prep(&mut lines);
        // Create a unit for this file
        let unit = Unit::new(path.to_path_buf(), lines);
        // Ensure no duplicate symbols
        // TODO
        // Create translation unit
        self.units.push(unit);
        Ok(())
    }

    pub fn asm(&mut self) -> Result<(), Box<dyn Error>> {
        // Concatenate translation units
        // TODO: keep track of source file when concatenating
        let unit = self.units.pop().unwrap_or_else(|| Unit::default());
        let unit = self
            .units
            .clone()
            .into_iter()
            .try_fold(unit, Unit::concat)?;
        // Assemble unit into binary
        self.words = unit.asm()?;
        Ok(())
    }

    pub fn out(&self, out: &Path) -> io::Result<()> {
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
pub struct AsmError {
    err: Box<dyn Error>,
}

impl Display for AsmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{} {}",
            "error".red().bold(),
            ":".bold(),
            format!("{}", self.err).bold(),
        )
    }
}

impl Error for AsmError {}

impl From<Box<dyn Error>> for AsmError {
    fn from(err: Box<dyn Error>) -> Self {
        Self { err }
    }
}

#[derive(Debug)]
pub struct VerboseError {
    err: AsmError,
    loc: (PathBuf, usize),
    line: String,
}

impl Display for VerboseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lineno = format!("{}", self.loc.1);
        writeln!(f, "{}", self.err)?;
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

impl Error for VerboseError {}

#[cfg(test)]
mod tests {}
