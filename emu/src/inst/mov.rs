use super::Instruction;
use crate::{iarch, uarch, Processor};

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
    isimm: bool,
    imm: uarch,
    mode: Mode,
}

impl Instruction for Mov {
    fn new(word: uarch) -> Self {
        assert_eq!((word >> 12), 0b1010);
        Self {
            op1: ((word >> 8) & 0xf) as usize,
            op2: ((word >> 0) & 0xf) as usize,
            isimm: (word & 0x0080) != 0,
            imm: word & 0x7f,
            mode: match (word >> 4) & 0x3 {
                0b00 => Mode::Mov,
                0b01 => Mode::Neg,
                0b10 => Mode::Not,
                _ => panic!(),
            },
        }
    }

    fn execute(&self, proc: &mut Processor) {
        println!("{:?}", self);
        let op2 = if self.isimm {
            self.imm
        } else {
            *proc.regs[self.op2]
        };
        let op2 = match self.mode {
            Mode::Mov => op2,
            Mode::Neg => !op2,
            Mode::Not => -(op2 as iarch) as uarch,
        };
        *proc.regs[self.op1] = op2;
    }
}
