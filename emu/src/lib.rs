//! # Emulator
//!
//! `emu` is an emulator for the KAP-16 microprocessor.

use std::fmt::{self, Debug, Display};
use std::fs::File;
use std::io::Read;
use std::mem;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::path::Path;

mod inst;
use inst::Instruction;

#[allow(non_camel_case_types)]
type iarch = i16;
#[allow(non_camel_case_types)]
type uarch = u16;

const ARCHSIZE: usize = mem::size_of::<uarch>();
const RAMSIZE: usize = 0x4000;

pub struct Emulator {
    proc: Processor,
}

impl Emulator {
    pub fn new() -> Self {
        Emulator {
            proc: Processor::new(),
        }
    }

    pub fn load<P>(&mut self, file: P) -> std::io::Result<()>
    where
        P: AsRef<Path>,
    {
        let mut f = File::open(file)?;
        f.read_exact(&mut self.proc.ram.0)
    }

    pub fn main(&mut self) {
        loop {
            let instr = self.proc.cycle();
            println!("{}", instr);
            // XXX: enable to print all registers
            if false {
                for (i, reg) in self.proc.regs.iter().enumerate() {
                    print!("R{:02}: {}", i, reg);
                    if (i + 1) % 4 != 0 {
                        print!(", ");
                    } else {
                        println!();
                    }
                }
                println!("SR : {:04b}", self.proc.sr.0);
                println!();
            }
        }
    }
}

impl Default for Emulator {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Default)]
pub struct Processor {
    regs: [Register; 16],
    sr: Register,
    ram: Ram,
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
        let word = self.ram[pc];
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
struct Ram([u8; RAMSIZE]);

impl Display for Ram {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const ROWSIZE: usize = mem::size_of::<usize>();
        for (i, row) in self.chunks(ROWSIZE).enumerate() {
            if i != 0 {
                writeln!(f)?;
            }
            write!(f, "{:#06x}:", ARCHSIZE * ROWSIZE * i)?;
            for word in row {
                write!(f, " {:04x}", word)?;
            }
        }
        write!(f, "")
    }
}

impl Default for Ram {
    fn default() -> Self {
        Self([0; RAMSIZE])
    }
}

impl Deref for Ram {
    type Target = [uarch];

    fn deref(&self) -> &Self::Target {
        unsafe { self.0.align_to::<uarch>().1 }
    }
}

impl DerefMut for Ram {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.align_to_mut::<uarch>().1 }
    }
}

impl Index<usize> for Ram {
    type Output = uarch;

    fn index(&self, idx: usize) -> &Self::Output {
        assert!((idx % 2) == 0);
        unsafe { &self.0.align_to::<uarch>().1[idx / ARCHSIZE] }
    }
}

impl IndexMut<usize> for Ram {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        assert!((idx % 2) == 0);
        unsafe { &mut self.0.align_to_mut::<uarch>().1[idx / ARCHSIZE] }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
