use std::fmt::{self, Display};

use super::Instruction;
use crate::{iarch, uarch, util, Processor};

#[derive(Debug)]
enum Mode {
    Mov,
    Neg,
    Not,
}

#[derive(Debug)]
pub struct Mov {
    op1: usize,
    op2: usize,
    imm: Option<iarch>,
    mode: Mode,
}

impl Display for Mov {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let label = format!("{:?}", self.mode).to_lowercase();
        let op1 = format!("r{}", self.op1);
        let op2 = match self.imm {
            Some(imm) => format!("{:#06x}", imm),
            None => format!("r{}", self.op2),
        };
        write!(f, "{} {}, {}", label, op1, op2)
    }
}

impl Instruction for Mov {
    fn new(word: uarch) -> Self {
        assert_eq!((word >> 12), 0b1010);
        Self {
            op1: ((word >> 8) & 0xf) as usize,
            op2: (word & 0xf) as usize,
            imm: match (word & 0x0080) != 0 {
                true => Some(util::sign_extend::<7, { uarch::BITS }>(word & 0x7f) as iarch),
                false => None,
            },
            mode: match (word & 0x0080) != 0 {
                true => Mode::Mov,
                false => match (word >> 4) & 0x3 {
                    0b00 => Mode::Mov,
                    0b01 => Mode::Neg,
                    0b10 => Mode::Not,
                    _ => panic!(),
                },
            },
        }
    }

    fn execute(&self, proc: &mut Processor) {
        // Extract operands
        let op2 = self.imm.unwrap_or(*proc.regs[self.op2] as iarch);
        // Compute result
        let res = match self.mode {
            Mode::Mov => op2,
            Mode::Neg => !op2,
            Mode::Not => -op2,
        } as uarch;
        // Set result
        *proc.regs[self.op1] = res;
    }
}
