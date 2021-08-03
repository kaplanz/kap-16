use crate::{uarch, Processor};

mod mov;
use self::mov::Mov;
mod and;
use self::and::And;
mod orr;
use self::orr::Orr;
mod xor;
use self::xor::Xor;

pub trait Instruction {
    fn new(word: uarch) -> Self
    where
        Self: Sized;

    fn execute(&self, proc: &mut Processor);
}

pub fn decode(word: uarch) -> Box<dyn Instruction> {
    match word >> 12 {
        0b1010 => Box::from(Mov::new(word)), // 0xa => MOV
        0b0110 => Box::from(And::new(word)), // 0x6 => AND
        0b0100 => Box::from(Orr::new(word)), // 0x4 => ORR
        0b0101 => Box::from(Xor::new(word)), // 0x5 => XOR
        _ => panic!("Could not decode: {:#x}", word),
    }
}
