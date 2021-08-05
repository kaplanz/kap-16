use std::fmt::{Debug, Display};

use crate::{iarch, uarch, Processor};

mod mov;
use self::mov::Mov;
mod bra;
use self::bra::Bra;
mod ldr;
use self::ldr::Ldr;
mod str;
use self::str::Str;
mod cmp;
use self::cmp::Cmp;
mod shf;
use self::shf::Shf;
mod and;
use self::and::And;
mod orr;
use self::orr::Orr;
mod xor;
use self::xor::Xor;
mod add;
use self::add::Add;
mod sub;
use self::sub::Sub;
mod mul;
use self::mul::Mul;

pub trait Instruction: Debug + Display {
    fn new(word: uarch) -> Self
    where
        Self: Sized;

    fn execute(&self, proc: &mut Processor);
}

pub fn decode(word: uarch) -> Box<dyn Instruction> {
    match word >> 12 {
        0b0000..=0b0011 => Box::from(Cmp::new(word)), // 0x0..=0x3 => CMP
        0b0100 => Box::from(Orr::new(word)),          // 0x4 => ORR
        0b0101 => Box::from(Xor::new(word)),          // 0x5 => XOR
        0b0110 => Box::from(And::new(word)),          // 0x6 => AND
        0b0111 => Box::from(Mul::new(word)),          // 0x7 => MUL
        0b1000..=0b1001 => Box::from(Sub::new(word)), // 0x8..=0x9 => SUB
        0b1010 => Box::from(Mov::new(word)),          // 0xa => MOV
        0b1011 => Box::from(Ldr::new(word)),          // 0xb => LDR
        0b1100 => Box::from(Add::new(word)),          // 0xc => ADD
        0b1101 => Box::from(Str::new(word)),          // 0xd => STR
        0b1110 => Box::from(Shf::new(word)),          // 0xe => SHF
        0b1111 => Box::from(Bra::new(word)),          // 0xf => BRA
        _ => panic!("Could not decode: {:#06x}", word),
    }
}

pub fn sign_extend<const F: u32, const T: u32>(x: uarch) -> iarch {
    let i = T - F;
    ((x << i) as iarch) >> i
}
