use std::fmt::{self, Debug, Display};
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::slice;

use super::uarch;

#[derive(Copy, Clone, Debug, Default)]
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

#[derive(Debug)]
pub struct Bank<const N: usize>([Register; N]);

impl<const N: usize> Default for Bank<N> {
    fn default() -> Self {
        Self([Default::default(); N])
    }
}

impl<const N: usize> Deref for Bank<N> {
    type Target = [Register; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> DerefMut for Bank<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const N: usize> Index<uarch> for Bank<N> {
    type Output = Register;

    fn index(&self, idx: uarch) -> &Self::Output {
        assert!(idx < N as uarch);
        &self.0[idx as usize]
    }
}

impl<const N: usize> IndexMut<uarch> for Bank<N> {
    fn index_mut(&mut self, idx: uarch) -> &mut Self::Output {
        assert!(idx < N as uarch);
        &mut self.0[idx as usize]
    }
}

impl<'a, const N: usize> IntoIterator for &'a Bank<N> {
    type Item = &'a Register;
    type IntoIter = slice::Iter<'a, Register>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
