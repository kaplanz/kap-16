use std::fmt::{self, Debug, Display};
use std::ops::{Deref, DerefMut};

use super::uarch;

#[derive(Debug, Default)]
pub struct Register(uarch);

impl Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:04x}", self.0)
    }
}

impl Deref for Register {
    type Target = uarch;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Register {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
