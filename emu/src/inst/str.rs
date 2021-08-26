use std::fmt::{self, Display};

use super::{Instruction, Op2};
use crate::{iarch, uarch, util, Processor, WORDSIZE};

#[derive(Debug)]
pub struct Str {
    op1: uarch,
    op2: Op2,
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
            let op2 = match self.op2 {
                Op2::Op2(op2) => format!("r{}", op2),
                Op2::Imm(imm) => format!("{:+#07x}", imm),
            };
            write!(f, "{} {}, &{}", label, op1, op2)
        }
    }
}

impl From<uarch> for Str {
    fn from(word: uarch) -> Self {
        assert_eq!((word >> 12), 0b1101);
        Self {
            op1: (word & 0x0f00) >> 8,
            op2: match (word & 0x0080) == 0 {
                true => Op2::Op2(word & 0x000f),
                false => Op2::Imm(util::sign_extend::<8, { uarch::BITS }>(
                    (WORDSIZE as uarch) * (word & 0x007f),
                )),
            },
            push: ((word ^ 0x0040) & 0x00c0) == 0,
        }
    }
}

impl From<Str> for uarch {
    fn from(instr: Str) -> Self {
        let mut word: uarch = 0;
        word |= 0b1101 << 12;
        word |= (instr.op1 << 8) & 0x0f00;
        word |= match instr.op2 {
            Op2::Op2(op2) => match instr.push {
                false => op2,
                true => 0x0040,
            },
            Op2::Imm(imm) => 0x0080 | (imm / (WORDSIZE as uarch)),
        } & 0x00ff;
        word
    }
}

impl Instruction for Str {
    fn execute(&self, proc: &mut Processor) {
        // Decrement frame pointer
        if self.push {
            *proc.regs[13] -= WORDSIZE as uarch;
        }
        // Compute result
        let res = match self.op2 {
            Op2::Op2(op2) => match self.push {
                false => *proc.regs[op2],
                true => *proc.regs[13],
            },
            Op2::Imm(imm) => (*proc.regs[15] as iarch + imm as iarch) as uarch,
        };
        // Set result
        proc.ram[res] = *proc.regs[self.op1];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sweep() {
        for mut word in 0xd000..=0xdfff {
            let instr = Str::from(word);
            if let Op2::Op2(_) = instr.op2 {
                word &= 0xffcf;
            }
            if instr.push {
                word &= 0xffc0;
            }
            let decoded: uarch = instr.into();
            assert_eq!(decoded, word);
        }
    }
}
