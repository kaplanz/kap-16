//! # Emulator
//!
//! `emu` is an emulator for the KAP-16 microprocessor.

use std::fmt::{self, Debug, Display};
use std::fs::File;
use std::io::Read;
use std::mem;
use std::ops::{Deref, DerefMut, Index, IndexMut};

mod inst;
use inst::Instruction;

#[allow(non_camel_case_types)]
type iarch = i16;
#[allow(non_camel_case_types)]
type uarch = u16;

const ARCHSIZE: usize = mem::size_of::<uarch>();
const ROMSIZE: usize = 0x1000;

pub struct Emulator {
    proc: Processor,
}

impl Emulator {
    pub fn new() -> Self {
        Emulator {
            proc: Processor::new(),
        }
    }

    pub fn main(&mut self, file: &str) {
        self.load(file).unwrap();
        self.run();
    }

    fn load(&mut self, file: &str) -> std::io::Result<()> {
        let mut f = File::open(file)?;
        f.read_exact(&mut self.proc.rom.0)
    }

    fn run(&mut self) {
        loop {
            let instr = self.proc.cycle();
            println!("{}", instr);
        }
    }
}

#[derive(Debug, Default)]
pub struct Processor {
    regs: [Register; 16],
    sr: Register,
    rom: Rom,
}

impl Processor {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    fn cycle(&mut self) -> Box<dyn Instruction> {
        let pc = *self.regs[15] as usize;
        *self.regs[15] += ARCHSIZE as uarch;
        let word = self.rom[pc];
        let instr = inst::decode(word);
        instr.execute(self);
        instr
    }
}

#[derive(Debug, Default)]
struct Register(uarch);

impl Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:04x}", self.0)
    }
}

impl Deref for Register {
    type Target = uarch;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Register {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug)]
struct Rom([u8; ROMSIZE]);

impl Display for Rom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const ROWSIZE: usize = mem::size_of::<usize>();
        for (i, row) in self.0.chunks(ROWSIZE).enumerate() {
            if i != 0 {
                writeln!(f)?;
            }
            write!(f, "{:#06x}:", ROWSIZE * i)?;
            for word in row {
                write!(f, " {:04x}", word)?;
            }
        }
        write!(f, "")
    }
}

impl Default for Rom {
    fn default() -> Self {
        Self([0; ROMSIZE])
    }
}

impl Index<usize> for Rom {
    type Output = uarch;

    fn index(&self, idx: usize) -> &Self::Output {
        assert!((idx % 2) == 0);
        unsafe { &self.0.align_to::<uarch>().1[idx / ARCHSIZE] }
    }
}

impl IndexMut<usize> for Rom {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        assert!((idx % 2) == 0);
        unsafe { &mut self.0.align_to_mut::<uarch>().1[idx / ARCHSIZE] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
