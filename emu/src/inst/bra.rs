use std::fmt::{self, Display};

use super::{Instruction, Op2};
use crate::{iarch, uarch, util, Processor, WORDSIZE};

#[derive(Debug)]
enum Cond {
    Ra = 0b000,
    Eq = 0b001,
    Ne = 0b010,
    Lt = 0b011,
    Le = 0b100,
    Ge = 0b101,
    Gt = 0b110,
}

#[derive(Debug)]
pub struct Bra {
    op2: Op2,
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
        let op2 = match self.op2 {
            Op2::Reg(op2) => format!("r{}", op2),
            Op2::Imm(imm) => format!("{:+#07x}", imm),
        };
        write!(f, "{} {}", label, op2)
    }
}

impl From<uarch> for Bra {
    fn from(word: uarch) -> Self {
        assert_eq!((word >> 12), 0b1111);
        Self {
            op2: match (word & 0x0080) == 0 {
                true => Op2::Reg(word & 0x000f),
                false => Op2::Imm(util::sign_extend::<8, { uarch::BITS }>(
                    (WORDSIZE as uarch) * (word & 0x007f),
                )),
            },
            link: (word & 0x0800) != 0,
            cond: match (word & 0x0700) >> 8 {
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
}

impl From<Bra> for uarch {
    fn from(instr: Bra) -> Self {
        let mut word: uarch = 0;
        word |= 0b1111 << 12;
        word |= ((instr.link as uarch) << 11) & 0x0800;
        word |= ((instr.cond as uarch) << 8) & 0x0700;
        word |= match instr.op2 {
            Op2::Reg(op2) => op2,
            Op2::Imm(imm) => 0x0080 | (imm / (WORDSIZE as uarch)),
        } & 0x00ff;
        word
    }
}

impl Instruction for Bra {
    fn execute(&self, proc: &mut Processor) {
        // Compute results
        let res = match self.op2 {
            Op2::Reg(op2) => *proc.regs[op2],
            Op2::Imm(imm) => (*proc.regs[15] as iarch + imm as iarch) as uarch,
        };
        let act = match self.cond {
            Cond::Ra => true,
            Cond::Eq => (*proc.sr & 0x0001) != 0, //                  Z
            Cond::Ne => (*proc.sr & 0x0001) == 0, //                 !Z
            Cond::Lt => (*proc.sr & 0x0002) != 0, //             N
            Cond::Le => (*proc.sr & 0x0003) != 0, //             N |  Z
            Cond::Ge => ((*proc.sr ^ 0x0002) & 0x0003) != 0, // !N |  Z
            Cond::Gt => (*proc.sr & 0x0003) == 0, //            !N & !Z
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sweep() {
        for mut word in 0xf000..=0xffff {
            match (word & 0x0700) >> 8 {
                0b111 => continue,
                _ => (),
            }
            let instr = Bra::from(word);
            if let Op2::Reg(_) = instr.op2 {
                word &= 0xff8f;
            }
            let decoded: uarch = instr.into();
            assert_eq!(decoded, word);
        }
    }
}
