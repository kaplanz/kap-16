use crate::uarch;

mod add;
mod and;
mod bra;
mod cmp;
mod ldr;
mod mov;
mod mul;
mod orr;
mod shf;
mod str;
mod sub;
mod xor;

#[derive(Debug)]
enum Op2 {
    Op2(uarch),
    Imm(uarch),
}
