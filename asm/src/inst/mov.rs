use std::cmp::Ordering;
use std::error::Error;
use std::fmt::{self, Display};
use std::str::FromStr;

use super::{Instruction, Op2, ParseInstructionError};
use crate::{lex, uarch, util};

#[derive(Debug)]
enum Mode {
    Mov = 0b00,
    Neg = 0b01,
    Not = 0b10,
}

#[derive(Debug)]
pub struct Mov {
    op1: uarch,
    op2: Op2,
    mode: Mode,
}

impl Display for Mov {
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

impl From<uarch> for Mov {
    fn from(word: uarch) -> Self {
        assert_eq!((word >> 12), 0b1010);
        Self {
            op1: (word & 0x0f00) >> 8,
            op2: match (word & 0x0080) == 0 {
                true => Op2::Reg(word & 0x000f),
                false => Op2::Imm(util::sign_extend::<7, { uarch::BITS }>(word & 0x007f)),
            },
            mode: match (word & 0x0080) != 0 {
                true => Mode::Mov,
                false => match (word & 0x0030) >> 4 {
                    0b00 => Mode::Mov,
                    0b01 => Mode::Neg,
                    0b10 => Mode::Not,
                    _ => panic!(),
                },
            },
        }
    }
}

impl From<Mov> for uarch {
    fn from(instr: Mov) -> Self {
        let mut word: uarch = 0;
        word |= 0b1010 << 12;
        word |= (instr.op1 << 8) & 0x0f00;
        word |= match instr.op2 {
            Op2::Reg(op2) => ((instr.mode as uarch) << 4) | op2,
            Op2::Imm(imm) => 0x0080 | imm,
        } & 0x00ff;
        word
    }
}

impl FromStr for Mov {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Only operate on lowercase strings
        // (also creates an owned String from &str)
        let s = s.to_lowercase();
        // Split into constituent tokens
        let tokens = lex::tokenize(s).ok_or(ParseInstructionError::EmptyStr)?;
        // Ensure correct number of tokens
        match tokens.len().cmp(&4) {
            Ordering::Less => Err(ParseInstructionError::MissingOps),
            Ordering::Equal => Ok(()),
            Ordering::Greater => Err(ParseInstructionError::ExtraOps),
        }?;
        // Parse mode
        let mode = match &*tokens[0] {
            "mov" => Mode::Mov,
            "neg" => Mode::Neg,
            "not" => Mode::Not,
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
            Op2::Imm(imm) if imm < 0x80 => Ok(()),
            _ => Err(ParseInstructionError::InvalidOp),
        }?;
        // Create Self from parts
        Ok(Self { op1, op2, mode })
    }
}

impl Instruction for Mov {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sweep() {
        for mut word in 0xa000..=0xafff {
            match (word & 0x00b0) >> 4 {
                0b0011 => continue,
                _ => (),
            }
            let instr = Mov::from(word);
            if let Op2::Reg(_) = instr.op2 {
                word &= 0xffbf;
            }
            let decoded: uarch = instr.into();
            assert_eq!(decoded, word);
        }
    }
}
