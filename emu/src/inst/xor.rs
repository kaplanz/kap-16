use std::fmt::{self, Display};

use super::Instruction;
use crate::{uarch, Processor};

#[derive(Debug)]
pub struct Xor {
    op1: usize,
    op2: usize,
    imm: Option<uarch>,
}

impl Display for Xor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let label = "xor";
        let op1 = format!("r{}", self.op1);
        let op2 = match self.imm {
            Some(imm) => format!("{:#06x}", imm),
            None => format!("r{}", self.op2),
        };
        write!(f, "{} {}, {}", label, op1, op2)
    }
}

impl Instruction for Xor {
    fn new(word: uarch) -> Self {
        assert_eq!((word >> 12), 0b0101);
        Self {
            op1: ((word >> 8) & 0xf) as usize,
            op2: (word & 0xf) as usize,
            imm: match (word & 0x0080) != 0 {
                true => Some(super::sign_extend::<7, { uarch::BITS }>(word & 0x7f) as uarch),
                false => None,
            },
        }
    }

    fn execute(&self, proc: &mut Processor) {
        // Extract operands
        let op1 = *proc.regs[self.op1];
        let op2 = self.imm.unwrap_or(*proc.regs[self.op2]);
        // Compute result
        let res = op1 ^ op2;
        // Compute condition codes
        let zero = res == 0;
        let negative = (res & 0x8000) != 0;
        // Set result, condition codes
        *proc.regs[self.op1] = res;
        *proc.sr ^= (*proc.sr & 0x0001) ^ (zero as uarch);
        *proc.sr ^= (*proc.sr & 0x0002) ^ ((negative as uarch) << 1);
        *proc.sr ^= *proc.sr & 0x0004;
        *proc.sr ^= *proc.sr & 0x0008;
    }
}
