use std::cmp::Ordering;
use std::error::Error;
use std::fmt::{self, Display};
use std::str::FromStr;

use super::{Instruction, Op2, ParseInstructionError};
use crate::{iarch, lex, uarch, util, WORDSIZE};

#[derive(Debug)]
enum Cond {
    Ra = 0b000,
    Eq = 0b001,
    Ne = 0b010,
    Lt = 0b011,
    Le = 0b100,
    Ge = 0b101,
    Gt = 0b110,
}

#[derive(Debug)]
pub struct Bra {
    op2: Op2,
    link: bool,
    cond: Cond,
}

impl Display for Bra {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let label = format!(
            "b{}{:?}",
            match self.link {
                true => "l",
                false => "",
            },
            self.cond
        )
        .to_lowercase();
        let op2 = match self.op2 {
            Op2::Reg(op2) => format!("r{}", op2),
            Op2::Imm(imm) => format!("{:+#07x}", imm),
        };
        write!(f, "{} {}", label, op2)
    }
}

impl From<uarch> for Bra {
    fn from(word: uarch) -> Self {
        assert_eq!((word >> 12), 0b1111);
        Self {
            op2: match (word & 0x0080) == 0 {
                true => Op2::Reg(word & 0x000f),
                false => Op2::Imm(util::sign_extend::<8, { uarch::BITS }>(
                    (WORDSIZE as uarch) * (word & 0x007f),
                )),
            },
            link: (word & 0x0800) != 0,
            cond: match (word & 0x0700) >> 8 {
                0b000 => Cond::Ra,
                0b001 => Cond::Eq,
                0b010 => Cond::Ne,
                0b011 => Cond::Lt,
                0b100 => Cond::Le,
                0b101 => Cond::Ge,
                0b110 => Cond::Gt,
                _ => panic!(),
            },
        }
    }
}

impl From<Bra> for uarch {
    fn from(instr: Bra) -> Self {
        let mut word: uarch = 0;
        word |= 0b1111 << 12;
        word |= ((instr.link as uarch) << 11) & 0x0800;
        word |= ((instr.cond as uarch) << 8) & 0x0700;
        word |= match instr.op2 {
            Op2::Reg(op2) => op2,
            Op2::Imm(imm) => 0x0080 | (imm / (WORDSIZE as uarch)),
        } & 0x00ff;
        word
    }
}

impl FromStr for Bra {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Only operate on lowercase strings
        // (also creates an owned String from &str)
        let s = s.to_lowercase();
        // Split into constituent tokens
        let tokens = lex::tokenize(&s).ok_or(ParseInstructionError::EmptyStr)?;
        // Ensure correct number of tokens
        match tokens.len().cmp(&2) {
            Ordering::Less => Err(ParseInstructionError::MissingOps),
            Ordering::Equal => Ok(()),
            Ordering::Greater => Err(ParseInstructionError::ExtraOps),
        }?;
        // Parse cond
        let cond = match &*tokens[0] {
            "b" | "bl" => Cond::Ra,
            "beq" | "bleq" => Cond::Eq,
            "bne" | "blne" => Cond::Ne,
            "blt" | "bllt" => Cond::Lt,
            "ble" | "blle" => Cond::Le,
            "bge" | "blge" => Cond::Ge,
            "bgt" | "blgt" => Cond::Gt,
            _ => Err(ParseInstructionError::BadInstruction)?,
        };
        // Parse link
        let link = tokens[0].len() % 2 == 0;
        // Parse op2
        let op2 = tokens[1].parse()?;
        // Ensure validity of ops
        match op2 {
            Op2::Reg(reg) if reg < 0x10 => Ok(()),
            Op2::Imm(imm) if (imm as iarch) < 0x80 && (imm as usize % WORDSIZE == 0) => Ok(()),
            _ => Err(ParseInstructionError::InvalidOp),
        }?;
        // Create Self from parts
        Ok(Self { op2, link, cond })
    }
}

impl Instruction for Bra {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sweep() {
        for mut word in 0xf000..=0xffff {
            match (word & 0x0700) >> 8 {
                0b111 => continue,
                _ => (),
            }
            let instr = Bra::from(word);
            if let Op2::Reg(_) = instr.op2 {
                word &= 0xff8f;
            }
            let decoded: uarch = instr.into();
            assert_eq!(decoded, word);
        }
    }
}
