use std::fmt::{self, Display};

use super::Instruction;
use crate::{iarch, uarch, util, Processor, WORDSIZE};

#[derive(Debug)]
pub struct Str {
    op1: usize,
    op2: usize,
    imm: Option<iarch>,
    push: bool,
}

impl Display for Str {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.push {
            let label = "push";
            let op1 = format!("r{}", self.op1);
            write!(f, "{} {}", label, op1)
        } else {
            let label = "str";
            let op1 = format!("r{}", self.op1);
            let op2 = match self.imm {
                Some(imm) => format!("{:+#07x}", imm),
                None => format!("r{}", self.op2),
            };
            write!(f, "{} {}, &{}", label, op1, op2)
        }
    }
}

impl Instruction for Str {
    fn new(word: uarch) -> Self {
        assert_eq!((word >> 12), 0b1101);
        Self {
            op1: ((word >> 8) & 0xf) as usize,
            op2: (word & 0xf) as usize,
            imm: match (word & 0x0080) != 0 {
                true => Some(util::sign_extend::<8, { uarch::BITS }>(
                    (WORDSIZE as uarch) * (word & 0x7f),
                ) as iarch),
                false => None,
            },
            push: ((word ^ 0x0040) & 0x00c0) == 0,
        }
    }

    fn execute(&self, proc: &mut Processor) {
        // Decrement frame pointer
        if self.push {
            *proc.regs[13] -= WORDSIZE as uarch;
        }
        // Compute result
        let res = match self.imm {
            Some(imm) => (*proc.regs[15] as iarch + imm) as uarch,
            None => match self.push {
                false => *proc.regs[self.op2],
                true => *proc.regs[13],
            },
        };
        // Set result
        proc.ram[res as usize] = *proc.regs[self.op1];
    }
}
