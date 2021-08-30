use std::fmt::{self, Display};
use std::str::FromStr;

use super::{Instruction, Op2, ParseInstructionError};
use crate::{lex, uarch, util};

#[derive(Debug)]
pub struct And {
    op1: uarch,
    op2: Op2,
}

impl And {
    pub const fn ident() -> &'static str {
        "and"
    }
}

impl Display for And {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let label = Self::ident();
        let op1 = format!("r{}", self.op1);
        let op2 = match self.op2 {
            Op2::Op2(op2) => format!("r{}", op2),
            Op2::Imm(imm) => format!("{:#06x}", imm),
        };
        write!(f, "{} {}, {}", label, op1, op2)
    }
}

impl From<uarch> for And {
    fn from(word: uarch) -> Self {
        assert_eq!((word >> 12), 0b0110);
        Self {
            op1: (word & 0x0f00) >> 8,
            op2: match (word & 0x0080) == 0 {
                true => Op2::Op2(word & 0x000f),
                false => Op2::Imm(util::sign_extend::<7, { uarch::BITS }>(word & 0x007f)),
            },
        }
    }
}

impl From<And> for uarch {
    fn from(instr: And) -> Self {
        let mut word: uarch = 0;
        word |= 0b0110 << 12;
        word |= (instr.op1 << 8) & 0x0f00;
        word |= match instr.op2 {
            Op2::Op2(op2) => op2,
            Op2::Imm(imm) => 0x0080 | imm,
        } & 0x00ff;
        word
    }
}

impl FromStr for And {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Only operate on lowercase strings
        // (also creates an owned String from &str)
        let s = s.to_lowercase();
        // Split into constituent tokens
        let tokens = lex::split(s).ok_or(Self::Err {})?;
        // Check instruction is correct
        (Self::ident() == tokens[0])
            .then(|| ())
            .ok_or(Self::Err {})?;
        // Parse op1
        let op1 = match tokens[1].split_at(1) {
            ("r", reg) => Ok(reg.parse()?),
            _ => Err(Self::Err {}),
        }?;
        // Look for "," separator
        ("," == tokens[2]).then(|| ()).ok_or(Self::Err {})?;
        // Parse op2
        let op2 = tokens[3].parse()?;
        // Ensure validity of ops
        (op1 < 0x10).then(|| ()).ok_or(Self::Err {})?;
        match op2 {
            Op2::Op2(reg) if reg < 0x10 => Ok(()),
            Op2::Imm(imm) if imm < 0x80 => Ok(()),
            _ => Err(Self::Err {}),
        }?;
        // Create Self from parts
        Ok(Self { op1, op2 })
    }
}

impl Instruction for And {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sweep() {
        for mut word in 0x6000..=0x6fff {
            let instr = And::from(word);
            if let Op2::Op2(_) = instr.op2 {
                word &= 0xff8f;
            }
            let decoded: uarch = instr.into();
            assert_eq!(decoded, word);
        }
    }
}
