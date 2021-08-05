use std::fmt::{self, Display};

use super::Instruction;
use crate::{iarch, uarch, Processor};

#[derive(Debug)]
pub struct Mul {
    op1: usize,
    op2: usize,
    imm: Option<iarch>,
}

impl Display for Mul {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let label = "mul";
        let op1 = format!("r{}", self.op1);
        let op2 = match self.imm {
            Some(imm) => format!("{:#06x}", imm),
            None => format!("r{}", self.op2),
        };
        write!(f, "{} {}, {}", label, op1, op2)
    }
}

impl Instruction for Mul {
    fn new(word: uarch) -> Self {
        assert_eq!((word >> 12), 0b0111);
        Self {
            op1: ((word >> 8) & 0xf) as usize,
            op2: ((word >> 0) & 0xf) as usize,
            imm: match (word & 0x0080) != 0 {
                true => Some(super::sign_extend::<7, { uarch::BITS }>(word & 0x7f)),
                false => None,
            },
        }
    }

    fn execute(&self, proc: &mut Processor) {
        // Extract operands
        let op1 = *proc.regs[self.op1] as iarch;
        let op2 = self.imm.unwrap_or(*proc.regs[self.op2] as iarch);
        // Calculate result, condition codes
        let (res, overflow) = op1.overflowing_mul(op2);
        let res = res as uarch;
        let zero = res == 0;
        let negative = (res & 0x8000) != 0;
        // Set result, condition codes
        *proc.regs[self.op1] = res;
        *proc.sr ^= (*proc.sr & 0x0001) ^ ((zero as uarch) << 0);
        *proc.sr ^= (*proc.sr & 0x0002) ^ ((negative as uarch) << 1);
        *proc.sr ^= (*proc.sr & 0x0004) ^ ((overflow as uarch) << 2);
        *proc.sr ^= (*proc.sr & 0x0008) ^ ((0 as uarch) << 3);
    }
}
