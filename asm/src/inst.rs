use std::fmt::{Debug, Display};
use std::num::ParseIntError;
use std::str::FromStr;
use std::{error, fmt};

use crate::uarch;

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

trait Instruction: Debug + Display + FromStr {}

#[derive(Clone, Debug)]
pub struct ParseInstructionError;

impl fmt::Display for ParseInstructionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "could not parse instruction")
    }
}

impl error::Error for ParseInstructionError {}

impl From<ParseIntError> for ParseInstructionError {
    fn from(_err: ParseIntError) -> Self {
        ParseInstructionError {}
    }
}

#[derive(Debug)]
enum Op2 {
    Op2(uarch),
    Imm(uarch),
}

impl FromStr for Op2 {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_at(1) {
            ("r", reg) => Ok(Op2::Op2(reg.parse()?)),
            ("0", imm) => Ok(Op2::Imm(match imm.split_at(1) {
                ("b", i) => uarch::from_str_radix(i, 2),
                ("d", i) => uarch::from_str_radix(i, 10),
                ("o", i) => uarch::from_str_radix(i, 8),
                ("x", i) => uarch::from_str_radix(i, 16),
                _ => return Err(ParseInstructionError {}),
            }?)),
            _ => Err(ParseInstructionError {}),
        }
    }
}

pub fn parse(line: &Vec<String>) -> Result<uarch, ParseInstructionError> {
    const ADD: &str = Add::ident();
    const AND: &str = And::ident();
    const MUL: &str = Mul::ident();
    const ORR: &str = Orr::ident();
    const XOR: &str = Xor::ident();
    match &*line[0] {
        ADD => Ok(line.join(" ").parse::<Add>()?.into()),
        AND => Ok(line.join(" ").parse::<And>()?.into()),
        "b" | "bl" | "beq" | "bleq" | "bne" | "blne" | "blt" | "bllt" | "ble" | "blle" | "bge"
        | "blge" | "bgt" | "blgt" => Ok(line.join(" ").parse::<Bra>()?.into()),
        "cmp" | "cmn" | "tst" | "teq" => Ok(line.join(" ").parse::<Cmp>()?.into()),
        "ldr" | "pop" => Ok(line.join(" ").parse::<Ldr>()?.into()),
        "mov" | "neg" | "not" => Ok(line.join(" ").parse::<Mov>()?.into()),
        MUL => Ok(line.join(" ").parse::<Mul>()?.into()),
        ORR => Ok(line.join(" ").parse::<Orr>()?.into()),
        "lsr" | "asr" | "ror" | "lsl" | "asl" | "rol" => Ok(line.join(" ").parse::<Shf>()?.into()),
        "str" | "push" => Ok(line.join(" ").parse::<Str>()?.into()),
        "sub" | "rsb" => Ok(line.join(" ").parse::<Sub>()?.into()),
        XOR => Ok(line.join(" ").parse::<Xor>()?.into()),
        _ => panic!("Could not parse: {:?}", line),
    }
}
