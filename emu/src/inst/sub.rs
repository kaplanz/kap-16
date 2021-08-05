use std::fmt::{self, Display};
use std::mem;

use super::Instruction;
use crate::{iarch, uarch, Processor};

#[derive(Debug)]
enum Mode {
    Sub,
    Rsb,
}

#[derive(Debug)]
pub struct Sub {
    op1: usize,
    op2: usize,
    imm: Option<uarch>,
    mode: Mode,
}

impl Display for Sub {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let label = format!("{:?}", self.mode).to_string().to_lowercase();
        let op1 = format!("r{}", self.op1);
        let op2 = match self.imm {
            Some(imm) => format!("{:#06x}", imm),
            None => format!("r{}", self.op2),
        };
        write!(f, "{} {}, {}", label, op1, op2)
    }
}

impl Instruction for Sub {
    fn new(word: uarch) -> Self {
        assert_eq!((word >> 13), 0b100);
        Self {
            op1: ((word >> 8) & 0xf) as usize,
            op2: ((word >> 0) & 0xf) as usize,
            imm: match (word & 0x0080) != 0 {
                true => Some(word & 0x7f),
                false => None,
            },
            mode: match (word & 0x1000) >> 12 {
                0b0 => Mode::Sub,
                0b1 => Mode::Rsb,
                _ => panic!(),
            },
        }
    }

    fn execute(&self, proc: &mut Processor) {
        // Extract operands
        let mut op1 = *proc.regs[self.op1] as iarch;
        let mut op2 = self.imm.unwrap_or(*proc.regs[self.op2]) as iarch;
        match self.mode {
            Mode::Sub => (),
            Mode::Rsb => mem::swap(&mut op1, &mut op2),
        }
        // Calculate result, condition codes
        let (res, overflow) = op1.overflowing_sub(op2);
        let res = res as uarch;
        let zero = res == 0;
        let negative = (res & 0x8000) != 0;
        let carry = overflow;
        // Set result, condition codes
        *proc.regs[self.op1] = res;
        *proc.sr ^= (*proc.sr & 0x0001) ^ ((zero as uarch) << 0);
        *proc.sr ^= (*proc.sr & 0x0002) ^ ((negative as uarch) << 1);
        *proc.sr ^= (*proc.sr & 0x0004) ^ ((overflow as uarch) << 2);
        *proc.sr ^= (*proc.sr & 0x0008) ^ ((carry as uarch) << 3);
    }
}
