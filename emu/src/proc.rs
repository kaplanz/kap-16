use std::fmt::{self, Display};

use super::{uarch, BANKSIZE, RAMSIZE, WORDSIZE};
use crate::inst::{self, Instruction};
use crate::ram::Ram;
use crate::reg::{Bank, Register};

#[derive(Debug, Default)]
pub struct Processor {
    pub regs: Bank<BANKSIZE>,
    pub sr: Register,
    pub ram: Ram<RAMSIZE>,
}

impl Processor {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn cycle(&mut self) -> Box<dyn Instruction> {
        let pc = *self.regs[15];
        *self.regs[15] += WORDSIZE as uarch;
        let word = self.ram[pc];
        let instr = inst::decode(word);
        instr.execute(self);
        instr
    }

    fn flags(&self) -> Vec<Flag> {
        let mut flags = Vec::new();
        (*self.sr & 0x0008 != 0).then(|| flags.push(Flag::Carry));
        (*self.sr & 0x0004 != 0).then(|| flags.push(Flag::Overflow));
        (*self.sr & 0x0002 != 0).then(|| flags.push(Flag::Negative));
        (*self.sr & 0x0001 != 0).then(|| flags.push(Flag::Zero));
        flags
    }
}

impl Display for Processor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, reg) in self.regs.iter().enumerate() {
            write!(f, "R{:02}: {}", i, reg)?;
            if (i + 1) % 4 != 0 {
                write!(f, ", ")?;
            } else {
                writeln!(f)?;
            }
        }
        write!(f, "SR : {}, {:?}", self.sr, self.flags())?;
        write!(f, "")
    }
}

#[derive(Debug)]
enum Flag {
    Carry,
    Overflow,
    Negative,
    Zero,
}
