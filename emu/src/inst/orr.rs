use super::Instruction;
use crate::{uarch, Processor};

#[derive(Debug)]
pub struct Orr {
    op1: usize,
    op2: usize,
    isimm: bool,
    imm: uarch,
}

impl Instruction for Orr {
    fn new(word: uarch) -> Self {
        assert_eq!((word >> 12), 0b0100);
        Self {
            op1: ((word >> 8) & 0xf) as usize,
            op2: ((word >> 0) & 0xf) as usize,
            isimm: (word & 0x0080) != 0,
            imm: word & 0x7f,
        }
    }

    fn execute(&self, proc: &mut Processor) {
        println!("{:?}", self);
        let op2 = if self.isimm {
            self.imm
        } else {
            *proc.regs[self.op2]
        };
        *proc.regs[self.op1] |= op2;
    }
}
