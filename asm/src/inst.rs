use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display};
use std::str::FromStr;

use crate::lex::ParseLexemeError;
use crate::{lex, uarch};

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
pub enum ParseInstructionError {
    EmptyStr,
    MissingOps,
    ExtraOps,
    BadInstruction,
    ExpectedSep,
    InvalidOp,
}

impl Display for ParseInstructionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::EmptyStr => "Could not parse instruction from empty string",
                Self::MissingOps => "Missing operands",
                Self::ExtraOps => "Extra operands",
                Self::BadInstruction => "Bad instruction name",
                Self::ExpectedSep => "Expected separator between operands",
                Self::InvalidOp => "Invalid operand",
            }
        )
    }
}

impl Error for ParseInstructionError {}

#[derive(Debug)]
enum Op2 {
    Reg(uarch),
    Imm(uarch),
}

impl FromStr for Op2 {
    type Err = ParseLexemeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(
            match s.chars().next().ok_or(ParseLexemeError::EmptyToken)? {
                '0' => Op2::Imm(lex::parse_imm(s)?),
                _ => Op2::Reg(lex::parse_reg(s)?),
            },
        )
    }
}

pub fn parse(line: &Vec<String>) -> Result<uarch, Box<dyn Error>> {
    match &*line[0] {
        "add" => Ok(line.join(" ").parse::<Add>()?.into()),
        "and" => Ok(line.join(" ").parse::<And>()?.into()),
        "b" | "bl" | "beq" | "bleq" | "bne" | "blne" | "blt" | "bllt" | "ble" | "blle" | "bge"
        | "blge" | "bgt" | "blgt" => Ok(line.join(" ").parse::<Bra>()?.into()),
        "cmp" | "cmn" | "tst" | "teq" => Ok(line.join(" ").parse::<Cmp>()?.into()),
        "ldr" | "pop" => Ok(line.join(" ").parse::<Ldr>()?.into()),
        "mov" | "neg" | "not" => Ok(line.join(" ").parse::<Mov>()?.into()),
        "mul" => Ok(line.join(" ").parse::<Mul>()?.into()),
        "orr" => Ok(line.join(" ").parse::<Orr>()?.into()),
        "lsr" | "asr" | "ror" | "lsl" | "asl" | "rol" => Ok(line.join(" ").parse::<Shf>()?.into()),
        "str" | "push" => Ok(line.join(" ").parse::<Str>()?.into()),
        "sub" | "rsb" => Ok(line.join(" ").parse::<Sub>()?.into()),
        "xor" => Ok(line.join(" ").parse::<Xor>()?.into()),
        _ => panic!("Could not parse: {:?}", line),
    }
}
