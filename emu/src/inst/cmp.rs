use std::fmt::{self, Display};

use super::{Instruction, Op2};
use crate::{uarch, util, Processor};

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

impl Instruction for Cmp {
    fn execute(&self, proc: &mut Processor) {
        // Extract operands
        let op1 = *proc.regs[self.op1];
        let op2 = match self.op2 {
            Op2::Reg(op2) => *proc.regs[op2],
            Op2::Imm(imm) => imm,
        };
        // Compute result
        let (res, carryout) = match self.mode {
            Mode::Cmp => op1.overflowing_sub(op2),
            Mode::Cmn => op1.overflowing_add(op2),
            Mode::Tst => (op1 & op2, false),
            Mode::Teq => (op1 ^ op2, false),
        };
        let carryin = ((res ^ op1 ^ op2) & 0x8000) != 0;
        // Compute condition codes
        let zero = res == 0;
        let negative = (res & 0x8000) != 0;
        let overflow = match self.mode {
            Mode::Cmp | Mode::Cmn => carryout ^ carryin,
            Mode::Tst | Mode::Teq => false,
        };
        let carry = match self.mode {
            Mode::Cmp | Mode::Cmn => carryout,
            Mode::Tst | Mode::Teq => false,
        };
        // Set result, condition codes
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
