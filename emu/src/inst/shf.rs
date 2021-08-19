use std::fmt::{self, Display};

use super::Instruction;
use crate::{iarch, uarch, Processor};

#[derive(Clone, Copy, Debug)]
enum Mode {
    Lsr,
    Asr,
    Ror,
    Lsl,
    Asl,
    Rol,
}

#[derive(Debug)]
pub struct Shf {
    op1: usize,
    op2: usize,
    imm: Option<uarch>,
    mode: Mode,
}

impl Display for Shf {
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

impl Instruction for Shf {
    fn new(word: uarch) -> Self {
        assert_eq!((word >> 12), 0b1110);
        Self {
            op1: ((word >> 8) & 0xf) as usize,
            op2: (word & 0xf) as usize,
            imm: match (word & 0x0080) != 0 {
                true => Some(word & 0x0f),
                false => None,
            },
            mode: match (word >> 4) & 0x7 {
                0b000 => Mode::Lsr,
                0b001 => Mode::Asr,
                0b010 => Mode::Ror,
                0b100 => Mode::Lsl,
                0b101 => Mode::Asl,
                0b110 => Mode::Rol,
                _ => panic!(),
            },
        }
    }

    fn execute(&self, proc: &mut Processor) {
        // Extract operands
        let op1 = *proc.regs[self.op1];
        let op2 = self.imm.unwrap_or(*proc.regs[self.op2]);
        // Compute result
        let res = match self.mode {
            Mode::Lsr => op1.checked_shr(op2.into()).unwrap_or_default(),
            Mode::Lsl => op1.checked_shl(op2.into()).unwrap_or_default(),
            Mode::Asr => (op1 as iarch).checked_shr(op2.into()).unwrap_or_default() as uarch,
            Mode::Asl => (op1 as iarch).checked_shl(op2.into()).unwrap_or_default() as uarch,
            Mode::Ror => op1.rotate_right(op2.into()),
            Mode::Rol => op1.rotate_left(op2.into()),
        };
        let carryout = match self.mode {
            Mode::Lsr | Mode::Asr => (op1 & 0x0001) != 0,
            Mode::Lsl | Mode::Asl => (op1 & 0x8000) != 0,
            Mode::Ror | Mode::Rol => false,
        };
        // Compute condition codes
        // <https://stackoverflow.com/a/20109377>
        let zero = res == 0;
        let negative = (res & 0x8000) != 0;
        let overflow = carryout ^ negative;
        let carry = carryout;
        // Set result, condition codes
        *proc.regs[self.op1] = res;
        *proc.sr ^= (*proc.sr & 0x0001) ^ (zero as uarch);
        *proc.sr ^= (*proc.sr & 0x0002) ^ ((negative as uarch) << 1);
        *proc.sr ^= (*proc.sr & 0x0004) ^ ((overflow as uarch) << 2);
        *proc.sr ^= (*proc.sr & 0x0008) ^ ((carry as uarch) << 3);
    }
}
