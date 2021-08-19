use std::fmt::{self, Display};

use super::Instruction;
use crate::{uarch, Processor};

#[derive(Debug)]
enum Mode {
    Cmp,
    Cmn,
    Tst,
    Teq,
}

#[derive(Debug)]
pub struct Cmp {
    op1: usize,
    op2: usize,
    imm: Option<uarch>,
    mode: Mode,
}

impl Display for Cmp {
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

impl Instruction for Cmp {
    fn new(word: uarch) -> Self {
        assert_eq!((word >> 14), 0b00);
        Self {
            op1: ((word >> 8) & 0xf) as usize,
            op2: (word & 0xf) as usize,
            imm: match (word & 0x0080) != 0 {
                true => Some(super::sign_extend::<7, { uarch::BITS }>(word & 0x7f) as uarch),
                false => None,
            },
            mode: match (word >> 12) & 0x3 {
                0b00 => Mode::Cmp,
                0b01 => Mode::Cmn,
                0b10 => Mode::Tst,
                0b11 => Mode::Teq,
                _ => panic!(),
            },
        }
    }

    fn execute(&self, proc: &mut Processor) {
        // Extract operands
        let op1 = *proc.regs[self.op1];
        let op2 = self.imm.unwrap_or(*proc.regs[self.op2]);
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
