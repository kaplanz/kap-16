use std::cmp::Ordering;
use std::error::Error;
use std::fmt::{self, Display};
use std::str::FromStr;

use super::{Instruction, Op2, ParseInstructionError};
use crate::{lex, uarch};

#[derive(Clone, Copy, Debug)]
enum Mode {
    Lsr = 0b000,
    Asr = 0b001,
    Ror = 0b010,
    Lsl = 0b100,
    Asl = 0b101,
    Rol = 0b110,
}

#[derive(Debug)]
pub struct Shf {
    op1: uarch,
    op2: Op2,
    mode: Mode,
}

impl Display for Shf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let label = format!("{:?}", self.mode).to_lowercase();
        let op1 = format!("r{}", self.op1);
        let op2 = match self.op2 {
            Op2::Reg(op2) => format!("r{}", op2),
            Op2::Imm(imm) => format!("{:#06x}", imm),
        };
        write!(f, "{} {}, {}", label, op1, op2)
    }
}

impl From<uarch> for Shf {
    fn from(word: uarch) -> Self {
        assert_eq!((word >> 12), 0b1110);
        Self {
            op1: (word & 0x0f00) >> 8,
            op2: match (word & 0x0080) == 0 {
                true => Op2::Reg(word & 0x000f),
                false => Op2::Imm(word & 0x000f),
            },
            mode: match (word & 0x0070) >> 4 {
                0b000 => Mode::Lsr,
                0b001 => Mode::Asr,
                0b010 => Mode::Ror,
                0b100 => Mode::Lsl,
                0b101 => Mode::Asl,
                0b110 => Mode::Rol,
                _ => panic!(),
            },
        }
    }
}

impl From<Shf> for uarch {
    fn from(instr: Shf) -> Self {
        let mut word: uarch = 0;
        word |= 0b1110 << 12;
        word |= (instr.op1 << 8) & 0x0f00;
        word |= ((instr.mode as uarch) << 4) & 0x0070;
        word |= match instr.op2 {
            Op2::Reg(op2) => op2,
            Op2::Imm(imm) => 0x0080 | imm,
        } & 0x00ff;
        word
    }
}

impl FromStr for Shf {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Only operate on lowercase strings
        // (also creates an owned String from &str)
        let s = s.to_lowercase();
        // Split into constituent tokens
        let tokens = lex::tokenize(&s).ok_or(ParseInstructionError::EmptyStr)?;
        // Ensure correct number of tokens
        match tokens.len().cmp(&4) {
            Ordering::Less => Err(ParseInstructionError::MissingOps),
            Ordering::Equal => Ok(()),
            Ordering::Greater => Err(ParseInstructionError::ExtraOps),
        }?;
        // Parse mode
        let mode = match &*tokens[0] {
            "lsr" => Mode::Lsr,
            "asr" => Mode::Asr,
            "ror" => Mode::Ror,
            "lsl" => Mode::Lsl,
            "asl" => Mode::Asl,
            "rol" => Mode::Rol,
            _ => Err(ParseInstructionError::BadInstruction)?,
        };
        // Parse op1
        let op1 = lex::parse_reg(&tokens[1])?;
        // Look for "," separator
        (tokens[2] == ",")
            .then(|| ())
            .ok_or(ParseInstructionError::ExpectedSep)?;
        // Parse op2
        let op2 = tokens[3].parse()?;
        // Ensure validity of ops
        (op1 < 0x10)
            .then(|| ())
            .ok_or(ParseInstructionError::InvalidOp)?;
        match op2 {
            Op2::Reg(reg) if reg < 0x10 => Ok(()),
            Op2::Imm(imm) if imm < 0x10 => Ok(()),
            _ => Err(ParseInstructionError::InvalidOp),
        }?;
        // Create Self from parts
        Ok(Self { op1, op2, mode })
    }
}

impl Instruction for Shf {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sweep() {
        for word in 0xe000..=0xefff {
            match (word & 0x0030) >> 4 {
                0b11 => continue,
                _ => (),
            }
            let instr = Shf::from(word);
            let decoded: uarch = instr.into();
            assert_eq!(decoded, word);
        }
    }
}
