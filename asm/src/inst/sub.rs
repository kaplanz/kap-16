use std::fmt::{self, Display};

use super::Op2;
use crate::uarch;

#[derive(Debug)]
enum Mode {
    Sub = 0b0,
    Rsb = 0b1,
}

#[derive(Debug)]
pub struct Sub {
    op1: uarch,
    op2: Op2,
    mode: Mode,
}

impl Display for Sub {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let label = format!("{:?}", self.mode).to_lowercase();
        let op1 = format!("r{}", self.op1);
        let op2 = match self.op2 {
            Op2::Op2(op2) => format!("r{}", op2),
            Op2::Imm(imm) => format!("{:#06x}", imm),
        };
        write!(f, "{} {}, {}", label, op1, op2)
    }
}

impl From<uarch> for Sub {
    fn from(word: uarch) -> Self {
        assert_eq!((word >> 13), 0b100);
        Self {
            op1: (word & 0x0f00) >> 8,
            op2: match (word & 0x0080) == 0 {
                true => Op2::Op2(word & 0x000f),
                false => Op2::Imm(word & 0x007f),
            },
            mode: match (word & 0x1000) >> 12 {
                0b0 => Mode::Sub,
                0b1 => Mode::Rsb,
                _ => panic!(),
            },
        }
    }
}

impl From<Sub> for uarch {
    fn from(instr: Sub) -> Self {
        let mut word: uarch = 0;
        word |= 0b100 << 13;
        word |= ((instr.mode as uarch) << 12) & 0x1000;
        word |= (instr.op1 << 8) & 0x0f00;
        word |= match instr.op2 {
            Op2::Op2(op2) => op2,
            Op2::Imm(imm) => 0x0080 | imm,
        } & 0x00ff;
        word
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sweep() {
        for mut word in 0x8000..=0x9fff {
            let instr = Sub::from(word);
            if let Op2::Op2(_) = instr.op2 {
                word &= 0xff8f;
            }
            let decoded: uarch = instr.into();
            assert_eq!(decoded, word);
        }
    }
}
