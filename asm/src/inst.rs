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
    UnknownInstruction,
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
                Self::UnknownInstruction => "Unknown instruction",
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

pub fn assemble(line: &Vec<String>) -> Result<uarch, Box<dyn Error>> {
    Ok(match &*line[0] {
        "add" => line.join(" ").parse::<Add>()?.into(),
        "and" => line.join(" ").parse::<And>()?.into(),
        "b" | "bl" | "beq" | "bleq" | "bne" | "blne" | "blt" | "bllt" | "ble" | "blle" | "bge"
        | "blge" | "bgt" | "blgt" => line.join(" ").parse::<Bra>()?.into(),
        "cmp" | "cmn" | "tst" | "teq" => line.join(" ").parse::<Cmp>()?.into(),
        "ldr" | "pop" => line.join(" ").parse::<Ldr>()?.into(),
        "mov" | "neg" | "not" => line.join(" ").parse::<Mov>()?.into(),
        "mul" => line.join(" ").parse::<Mul>()?.into(),
        "orr" => line.join(" ").parse::<Orr>()?.into(),
        "lsr" | "asr" | "ror" | "lsl" | "asl" | "rol" => line.join(" ").parse::<Shf>()?.into(),
        "str" | "push" => line.join(" ").parse::<Str>()?.into(),
        "sub" | "rsb" => line.join(" ").parse::<Sub>()?.into(),
        "xor" => line.join(" ").parse::<Xor>()?.into(),
        _ => Err(ParseInstructionError::UnknownInstruction)?,
    })
}
