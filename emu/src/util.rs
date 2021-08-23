use super::{iarch, uarch};

pub fn sign_extend<const F: u32, const T: u32>(x: uarch) -> uarch {
    assert!(T > F);
    let i = T - F;
    (((x << i) as iarch) >> i) as uarch
}
