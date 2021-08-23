use std::fmt::{self, Display};

use super::{Instruction, Op2};
use crate::{iarch, uarch, util, Processor};

#[derive(Debug)]
enum Mode {
    Mov = 0b00,
    Neg = 0b01,
    Not = 0b10,
}

#[derive(Debug)]
pub struct Mov {
    op1: uarch,
    op2: Op2,
    mode: Mode,
}

impl Display for Mov {
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

impl From<uarch> for Mov {
    fn from(word: uarch) -> Self {
        assert_eq!((word >> 12), 0b1010);
        Self {
            op1: (word & 0x0f00) >> 8,
            op2: match (word & 0x0080) == 0 {
                true => Op2::Op2(word & 0x000f),
                false => Op2::Imm(util::sign_extend::<7, { uarch::BITS }>(word & 0x007f)),
            },
            mode: match (word & 0x0080) != 0 {
                true => Mode::Mov,
                false => match (word & 0x0030) >> 4 {
                    0b00 => Mode::Mov,
                    0b01 => Mode::Neg,
                    0b10 => Mode::Not,
                    _ => panic!(),
                },
            },
        }
    }
}

impl From<Mov> for uarch {
    fn from(instr: Mov) -> Self {
        let mut word: uarch = 0;
        word |= 0b1010 << 12;
        word |= instr.op1 << 8;
        word |= (instr.mode as uarch) << 4;
        word |= match instr.op2 {
            Op2::Op2(op2) => op2,
            Op2::Imm(imm) => 0x0080 | imm,
        };
        word
    }
}

impl Instruction for Mov {
    fn execute(&self, proc: &mut Processor) {
        // Extract operands
        let op2 = match self.op2 {
            Op2::Op2(op2) => *proc.regs[op2],
            Op2::Imm(imm) => imm,
        };
        // Compute result
        let res = match self.mode {
            Mode::Mov => op2,
            Mode::Neg => !op2,
            Mode::Not => -(op2 as iarch) as uarch,
        } as uarch;
        // Set result
        *proc.regs[self.op1] = res;
    }
}
