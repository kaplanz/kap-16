use std::fmt::{self, Display};

use super::{Instruction, Op2};
use crate::{uarch, Processor};

#[derive(Debug)]
pub struct Add {
    op1: uarch,
    op2: Op2,
}

impl Display for Add {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let label = "add";
        let op1 = format!("r{}", self.op1);
        let op2 = match self.op2 {
            Op2::Op2(op2) => format!("r{}", op2),
            Op2::Imm(imm) => format!("{:#06x}", imm),
        };
        write!(f, "{} {}, {}", label, op1, op2)
    }
}

impl From<uarch> for Add {
    fn from(word: uarch) -> Self {
        assert_eq!((word >> 12), 0b1100);
        Self {
            op1: (word & 0x0f00) >> 8,
            op2: match (word & 0x0080) == 0 {
                true => Op2::Op2(word & 0x000f),
                false => Op2::Imm(word & 0x007f),
            },
        }
    }
}

impl From<Add> for uarch {
    fn from(instr: Add) -> Self {
        let mut word: uarch = 0;
        word |= 0b1100 << 12;
        word |= instr.op1 << 8;
        word |= match instr.op2 {
            Op2::Op2(op2) => op2,
            Op2::Imm(imm) => 0x0080 | imm,
        };
        word
    }
}

impl Instruction for Add {
    fn execute(&self, proc: &mut Processor) {
        // Extract operands
        let op1 = *proc.regs[self.op1];
        let op2 = match self.op2 {
            Op2::Op2(op2) => *proc.regs[op2],
            Op2::Imm(imm) => imm,
        };
        // Compute result
        let (res, carryout) = op1.overflowing_add(op2);
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
