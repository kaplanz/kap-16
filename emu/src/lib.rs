//! # Emulator
//!
//! `emu` is an emulator for the KAP-16 microprocessor.

use std::fmt::{self, Debug};
use std::fs::File;
use std::io::Read;
use std::mem;
use std::ops::{Deref, DerefMut};

mod inst;

#[allow(non_camel_case_types)]
type iarch = i16;
#[allow(non_camel_case_types)]
type uarch = u16;

const ROMSIZE: usize = 0x0020;

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
        let rom = unsafe {
            const U8ROMSIZE: usize = mem::size_of::<uarch>() * ROMSIZE;
            mem::transmute::<&mut [uarch; ROMSIZE], &mut [u8; U8ROMSIZE]>(&mut self.proc.rom.0)
        };
        f.read_exact(rom)
    }

    fn run(&mut self) {
        for i in 0..ROMSIZE {
            let instr = inst::decode(self.proc.rom.0[i]);
            instr.execute(&mut self.proc);
        }
    }
}

pub struct Processor {
    regs: [Register; 16],
    rom: Rom,
}

impl Processor {
    fn new() -> Self {
        Self {
            regs: Default::default(),
            rom: Rom([0; ROMSIZE]),
        }
    }
}

#[derive(Default)]
struct Register(uarch);

impl Debug for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

struct Rom([uarch; ROMSIZE]);

impl Debug for Rom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const CHUNK: usize = 4;
        for (i, data) in self.0.chunks(CHUNK).enumerate() {
            if i != 0 {
                writeln!(f)?;
            }
            write!(f, "{:#06x}:", CHUNK * i)?;
            for word in data {
                write!(f, " {:04x}", word)?;
            }
        }
        write!(f, "")
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
