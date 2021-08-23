use std::fmt::{Debug, Display};

use crate::{uarch, Processor};

mod add;
mod and;
mod bra;
mod cmp;
mod ldr;
mod mov;
mod mul;
mod orr;
mod shf;
mod str;
mod sub;
mod xor;

use self::add::Add;
use self::and::And;
use self::bra::Bra;
use self::cmp::Cmp;
use self::ldr::Ldr;
use self::mov::Mov;
use self::mul::Mul;
use self::orr::Orr;
use self::shf::Shf;
use self::str::Str;
use self::sub::Sub;
use self::xor::Xor;

pub trait Instruction
where
    Self: Debug + Display,
{
    fn execute(&self, proc: &mut Processor);
}

#[derive(Debug)]
enum Op2 {
    Op2(uarch),
    Imm(uarch),
}

pub fn decode(word: uarch) -> Box<dyn Instruction> {
    match word >> 12 {
        0b0000..=0b0011 => Box::from(Cmp::from(word)), // 0x0..=0x3 => CMP
        0b0100 => Box::from(Orr::from(word)),          // 0x4       => ORR
        0b0101 => Box::from(Xor::from(word)),          // 0x5       => XOR
        0b0110 => Box::from(And::from(word)),          // 0x6       => AND
        0b0111 => Box::from(Mul::from(word)),          // 0x7       => MUL
        0b1000..=0b1001 => Box::from(Sub::from(word)), // 0x8..=0x9 => SUB
        0b1010 => Box::from(Mov::from(word)),          // 0xa       => MOV
        0b1011 => Box::from(Ldr::from(word)),          // 0xb       => LDR
        0b1100 => Box::from(Add::from(word)),          // 0xc       => ADD
        0b1101 => Box::from(Str::from(word)),          // 0xd       => STR
        0b1110 => Box::from(Shf::from(word)),          // 0xe       => SHF
        0b1111 => Box::from(Bra::from(word)),          // 0xf       => BRA
        _ => panic!("Could not decode: {:#06x}", word),
    }
}
