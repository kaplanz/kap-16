//! # Emulator
//!
//! `emu` is an emulator for the KAP-16 microprocessor.

use std::fs::File;
use std::io::{self, Read};
use std::mem;
use std::path::Path;

mod inst;
mod proc;
mod ram;
mod reg;
mod util;

use self::proc::Processor;

#[allow(non_camel_case_types)]
type iarch = i16;
#[allow(non_camel_case_types)]
type uarch = u16;

const BANKSIZE: usize = 0x10;
const RAMSIZE: usize = 0x4000;
const WORDSIZE: usize = mem::size_of::<uarch>();

#[derive(Default)]
pub struct Emulator {
    proc: Processor,
}

impl Emulator {
    pub fn new() -> Self {
        Self {
            proc: Processor::new(),
        }
    }

    pub fn load(&mut self, file: &Path) -> io::Result<()> {
        // Open the ROM file
        let mut f = File::open(file)?;

        // Read its contents into memory
        let buf = &mut self.proc.ram.0;
        let read = f.read(buf)?;

        // Error checking
        if read < buf.len() {
            log::warn!(
                "Read {} bytes from {:?}; zero padded remaining {} bytes.",
                read,
                file,
                buf.len() - read
            );
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
