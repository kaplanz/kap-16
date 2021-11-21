use std::cmp::Ordering;
use std::error::Error;
use std::fmt::{self, Display};
use std::str::FromStr;

use super::{Instruction, InstructionError, Op2};
use crate::{lex, uarch, util};

#[derive(Debug)]
enum Mode {
    Cmp = 0b00,
    Cmn = 0b01,
    Tst = 0b10,
    Teq = 0b11,
}

#[derive(Debug)]
pub struct Cmp {
    op1: uarch,
    op2: Op2,
    mode: Mode,
}

impl Display for Cmp {
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

impl From<uarch> for Cmp {
    fn from(word: uarch) -> Self {
        assert_eq!((word >> 14), 0b00);
        Self {
            op1: (word & 0x0f00) >> 8,
            op2: match (word & 0x0080) == 0 {
                true => Op2::Reg(word & 0x000f),
                false => Op2::Imm(util::sign_extend::<7, { uarch::BITS }>(word & 0x007f)),
            },
            mode: match (word & 0x3000) >> 12 {
                0b00 => Mode::Cmp,
                0b01 => Mode::Cmn,
                0b10 => Mode::Tst,
                0b11 => Mode::Teq,
                _ => panic!(),
            },
        }
    }
}

impl From<Cmp> for uarch {
    fn from(instr: Cmp) -> Self {
        let mut word: uarch = 0;
        word |= 0b00 << 14;
        word |= ((instr.mode as uarch) << 12) & 0x3000;
        word |= (instr.op1 << 8) & 0x0f00;
        word |= match instr.op2 {
            Op2::Reg(op2) => op2,
            Op2::Imm(imm) => 0x0080 | imm,
        } & 0x00ff;
        word
    }
}

impl FromStr for Cmp {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Only operate on lowercase strings
        // (also creates an owned String from &str)
        let s = s.to_lowercase();
        // Split into constituent tokens
        let tokens = lex::tokenize(&s).ok_or(InstructionError::EmptyStr)?;
        // Ensure correct number of tokens
        match tokens.len().cmp(&4) {
            Ordering::Less => Err(InstructionError::MissingOps),
            Ordering::Equal => Ok(()),
            Ordering::Greater => Err(InstructionError::ExtraOps),
        }?;
        // Parse mode
        let mode = match &*tokens[0] {
            "cmp" => Mode::Cmp,
            "cmn" => Mode::Cmn,
            "tst" => Mode::Tst,
            "teq" => Mode::Teq,
            _ => return Err(InstructionError::BadInstruction.into()),
        };
        // Parse op1
        let op1 = lex::parse_reg(&tokens[1])?;
        // Look for "," separator
        (tokens[2] == ",")
            .then(|| ())
            .ok_or(InstructionError::ExpectedSep)?;
        // Parse op2
        let op2 = tokens[3].parse()?;
        // Ensure validity of ops
        (op1 < 0x10)
            .then(|| ())
            .ok_or(InstructionError::InvalidOp)?;
        match op2 {
            Op2::Reg(reg) if reg < 0x10 => Ok(()),
            Op2::Imm(imm) if imm < 0x80 => Ok(()),
            _ => Err(InstructionError::InvalidOp),
        }?;
        // Create Self from parts
        Ok(Self { op1, op2, mode })
    }
}

impl Instruction for Cmp {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sweep() {
        for mut word in 0x0000..=0x3fff {
            let instr = Cmp::from(word);
            if let Op2::Reg(_) = instr.op2 {
                word &= 0xff8f;
            }
            let decoded: uarch = instr.into();
            assert_eq!(decoded, word);
        }
    }
}
