//! # Emulator
//!
//! `emu` is an emulator for the KAP-16 microprocessor.

use std::fmt::{self, Debug, Display};
use std::fs::File;
use std::io::{self, Read};
use std::mem;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::path::Path;

use log;

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

    pub fn load<P>(&mut self, file: P) -> io::Result<()>
    where
        P: AsRef<Path> + Debug,
    {
        // Open the ROM file
        let mut f = File::open(&file)?;

        // Read its contents into memory
        let buf = &mut self.proc.ram.0;
        let read = f.read(buf)?;

        // Error checking
        if read < buf.len() {
            log::warn!("Read {} bytes from {:?}; padded with zeros.", read, &file);
        } else if (buf.len() as u64) < f.metadata()?.len() {
            log::error!(
                "Read {} bytes from {:?}; truncated remaining {} bytes.",
                read,
                &file,
                f.metadata()?.len() - (read as u64),
            );
        }

        Ok(())
    }

    pub fn main(&mut self) {
        loop {
            let instr = self.proc.cycle();
            log::info!("{}", instr);
            log::debug!("{}", self.proc);
            log::trace!("{}", self.proc.ram);
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

    fn flags(&self) -> Vec<Flag> {
        let mut flags = Vec::new();
        (self.sr.0 & 0x0008 != 0).then(|| flags.push(Flag::Carry));
        (self.sr.0 & 0x0004 != 0).then(|| flags.push(Flag::Overflow));
        (self.sr.0 & 0x0002 != 0).then(|| flags.push(Flag::Negative));
        (self.sr.0 & 0x0001 != 0).then(|| flags.push(Flag::Zero));
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
            if row.iter().all(|&word| word == 0) {
                continue;
            }
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
