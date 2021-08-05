use std::fmt::{self, Display};

use super::Instruction;
use crate::{uarch, Processor};

#[derive(Debug)]
pub struct Add {
    op1: usize,
    op2: usize,
    imm: Option<uarch>,
}

impl Display for Add {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let label = "add";
        let op1 = format!("r{}", self.op1);
        let op2 = match self.imm {
            Some(imm) => format!("{:#06x}", imm),
            None => format!("r{}", self.op2),
        };
        write!(f, "{} {}, {}", label, op1, op2)
    }
}

impl Instruction for Add {
    fn new(word: uarch) -> Self {
        assert_eq!((word >> 12), 0b1100);
        Self {
            op1: ((word >> 8) & 0xf) as usize,
            op2: ((word >> 0) & 0xf) as usize,
            imm: match (word & 0x0080) != 0 {
                true => Some(word & 0x7f),
                false => None,
            },
        }
    }

    fn execute(&self, proc: &mut Processor) {
        // Extract operands
        let op1 = *proc.regs[self.op1];
        let op2 = self.imm.unwrap_or(*proc.regs[self.op2]);
        // Calculate result, condition codes
        let (res, overflow) = op1.overflowing_add(op2);
        let zero = res == 0;
        let carry = overflow;
        let negative = (res & 0x8000) != 0;
        // Set result, condition codes
        *proc.regs[self.op1] = res;
        *proc.sr ^= (*proc.sr & 0x0001) ^ ((overflow as uarch) << 0);
        *proc.sr ^= (*proc.sr & 0x0002) ^ ((zero as uarch) << 1);
        *proc.sr ^= (*proc.sr & 0x0004) ^ ((carry as uarch) << 2);
        *proc.sr ^= (*proc.sr & 0x0008) ^ ((negative as uarch) << 3);
    }
}
