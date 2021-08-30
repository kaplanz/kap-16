use std::fmt::{self, Display};
use std::mem;

use super::{Instruction, Op2};
use crate::{uarch, Processor};

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
            Op2::Reg(op2) => format!("r{}", op2),
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
                true => Op2::Reg(word & 0x000f),
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
            Op2::Reg(op2) => op2,
            Op2::Imm(imm) => 0x0080 | imm,
        } & 0x00ff;
        word
    }
}

impl Instruction for Sub {
    fn execute(&self, proc: &mut Processor) {
        // Extract operands
        let mut op1 = *proc.regs[self.op1];
        let mut op2 = match self.op2 {
            Op2::Reg(op2) => *proc.regs[op2],
            Op2::Imm(imm) => imm,
        };
        match self.mode {
            Mode::Sub => (),
            Mode::Rsb => mem::swap(&mut op1, &mut op2),
        }
        // Compute result
        let (res, carryout) = op1.overflowing_sub(op2);
        let carryin = ((res ^ op1 ^ op2) & 0x8000) != 0;
        // Compute condition codes
        let zero = res == 0;
        let negative = (res & 0x8000) != 0;
        let overflow = carryout ^ carryin;
        let carry = carryout;
        // Set result, condition codes
        *proc.regs[self.op1] = res;
        *proc.sr ^= (*proc.sr & 0x0001) ^ (zero as uarch);
        *proc.sr ^= (*proc.sr & 0x0002) ^ ((negative as uarch) << 1);
        *proc.sr ^= (*proc.sr & 0x0004) ^ ((overflow as uarch) << 2);
        *proc.sr ^= (*proc.sr & 0x0008) ^ ((carry as uarch) << 3);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sweep() {
        for mut word in 0x8000..=0x9fff {
            let instr = Sub::from(word);
            if let Op2::Reg(_) = instr.op2 {
                word &= 0xff8f;
            }
            let decoded: uarch = instr.into();
            assert_eq!(decoded, word);
        }
    }
}
