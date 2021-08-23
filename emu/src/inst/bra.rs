use std::fmt::{self, Display};

use super::Instruction;
use crate::{iarch, uarch, util, Processor, WORDSIZE};

#[derive(Debug)]
enum Cond {
    Ra,
    Eq,
    Ne,
    Lt,
    Le,
    Ge,
    Gt,
}

#[derive(Debug)]
pub struct Bra {
    op2: usize,
    imm: Option<iarch>,
    link: bool,
    cond: Cond,
}

impl Display for Bra {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let label = format!(
            "b{}{:?}",
            match self.link {
                true => "l",
                false => "",
            },
            self.cond
        )
        .to_lowercase();
        let op2 = match self.imm {
            Some(imm) => format!("{:+#07x}", imm),
            None => format!("r{}", self.op2),
        };
        write!(f, "{} {}", label, op2)
    }
}

impl Instruction for Bra {
    fn new(word: uarch) -> Self {
        assert_eq!((word >> 12), 0b1111);
        Self {
            op2: (word & 0xf) as usize,
            imm: match (word & 0x0080) != 0 {
                true => Some(util::sign_extend::<8, { uarch::BITS }>(
                    (WORDSIZE as uarch) * (word & 0x7f),
                ) as iarch),
                false => None,
            },
            link: (word & 0x0800) != 0,
            cond: match (word >> 8) & 0x7 {
                0b000 => Cond::Ra,
                0b001 => Cond::Eq,
                0b010 => Cond::Ne,
                0b011 => Cond::Lt,
                0b100 => Cond::Le,
                0b101 => Cond::Ge,
                0b110 => Cond::Gt,
                _ => panic!(),
            },
        }
    }

    fn execute(&self, proc: &mut Processor) {
        // Compute results
        let res = match self.imm {
            Some(imm) => (*proc.regs[15] as iarch + imm) as uarch,
            None => *proc.regs[self.op2],
        };
        let act = match self.cond {
            Cond::Ra => true,
            Cond::Eq => (*proc.sr & 0x0001) != 0, //  Z
            Cond::Ne => (*proc.sr & 0x0001) == 0, // !Z
            Cond::Lt => (*proc.sr & 0x0002) != 0, //  N
            Cond::Le => (*proc.sr & 0x0003) != 0, //  Z |  N
            Cond::Ge => ((*proc.sr ^ 0x0002) & 0x0003) != 0, //  Z | !N
            Cond::Gt => (*proc.sr & 0x0003) == 0, // !N & !Z
        };
        // Set result
        if act {
            if self.link {
                *proc.regs[14] = *proc.regs[15];
            }
            *proc.regs[15] = res;
        }
    }
}
