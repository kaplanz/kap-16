use std::cmp::Ordering;
use std::error::Error;
use std::fmt::{self, Display};
use std::str::FromStr;

use super::{Instruction, InstructionError, Op2};
use crate::{iarch, lex, uarch, util, WORDSIZE};

#[derive(Debug)]
enum Mode {
    Str = 0b0,
    Push = 0b1,
}

#[derive(Debug)]
pub struct Str {
    op1: uarch,
    op2: Op2,
    mode: Mode,
}

impl Display for Str {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let label = format!("{:?}", self.mode).to_lowercase();
        match self.mode {
            Mode::Str => {
                let op1 = format!("r{}", self.op1);
                let op2 = match self.op2 {
                    Op2::Reg(op2) => format!("r{}", op2),
                    Op2::Imm(imm) => format!("{:+#07x}", imm),
                };
                write!(f, "{} {}, &{}", label, op1, op2)
            }
            Mode::Push => {
                let op1 = format!("r{}", self.op1);
                write!(f, "{} {}", label, op1)
            }
        }
    }
}

impl From<uarch> for Str {
    fn from(word: uarch) -> Self {
        assert_eq!((word >> 12), 0b1101);
        Self {
            op1: (word & 0x0f00) >> 8,
            op2: match (word & 0x0080) == 0 {
                true => Op2::Reg(word & 0x000f),
                false => Op2::Imm(util::sign_extend::<8, { uarch::BITS }>(
                    (WORDSIZE as uarch) * (word & 0x007f),
                )),
            },
            mode: match ((word ^ 0x0040) & 0x00c0) == 0 {
                false => Mode::Str,
                true => Mode::Push,
            },
        }
    }
}

impl From<Str> for uarch {
    fn from(instr: Str) -> Self {
        let mut word: uarch = 0;
        word |= 0b1101 << 12;
        word |= (instr.op1 << 8) & 0x0f00;
        word |= match instr.op2 {
            Op2::Reg(op2) => match instr.mode {
                Mode::Str => op2,
                Mode::Push => 0x0040,
            },
            Op2::Imm(imm) => 0x0080 | (imm / (WORDSIZE as uarch)),
        } & 0x00ff;
        word
    }
}

impl FromStr for Str {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Only operate on lowercase strings
        // (also creates an owned String from &str)
        let s = s.to_lowercase();
        // Split into constituent tokens
        let tokens = lex::tokenize(&s).ok_or(InstructionError::EmptyStr)?;
        // Ensure at least one token
        (!tokens.is_empty())
            .then(|| ())
            .ok_or(InstructionError::MissingOps)?;
        // Parse mode
        let mode = match &*tokens[0] {
            "str" => Mode::Str,
            "push" => Mode::Push,
            _ => return Err(InstructionError::BadInstruction.into()),
        };
        let ntokens = match mode {
            Mode::Str => 4,
            Mode::Push => 2,
        };
        // Ensure correct number of tokens
        match tokens.len().cmp(&ntokens) {
            Ordering::Less => Err(InstructionError::MissingOps),
            Ordering::Equal => Ok(()),
            Ordering::Greater => Err(InstructionError::ExtraOps),
        }?;
        // Parse op1
        let op1 = lex::parse_reg(&tokens[1])?;
        // Ensure validity of op1
        (op1 < 0x10)
            .then(|| ())
            .ok_or(InstructionError::InvalidOp)?;
        // Parse for Mode::Str
        let op2 = match mode {
            Mode::Str => {
                // Look for "," separator
                (tokens[2] == ",")
                    .then(|| ())
                    .ok_or(InstructionError::ExpectedSep)?;
                // Parse op2
                let op2 = tokens[3].parse()?;
                // Ensure validity of op2
                match op2 {
                    Op2::Reg(reg) if reg < 0x10 => Ok(()),
                    Op2::Imm(imm) if (imm as iarch) < 0x80 && (imm as usize % WORDSIZE == 0) => {
                        Ok(())
                    }
                    _ => Err(InstructionError::InvalidOp),
                }?;
                op2
            }
            Mode::Push => Op2::Reg(Default::default()),
        };
        // Create Self from parts
        Ok(Self { op1, op2, mode })
    }
}

impl Instruction for Str {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sweep() {
        for mut word in 0xd000..=0xdfff {
            let instr = Str::from(word);
            if let Op2::Reg(_) = instr.op2 {
                word &= 0xffcf;
            }
            if let Mode::Push = instr.mode {
                word &= 0xffc0;
            }
            let decoded: uarch = instr.into();
            assert_eq!(decoded, word);
        }
    }
}
