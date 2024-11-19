mod fibonacci;
mod ranged_permutations;

pub use self::{fibonacci::Fibonacci, ranged_permutations::RangedPermutations};

pub trait Iterators: Iterator {
    fn ranged_permutations(self, length: usize) -> RangedPermutations<Self>
    where
        Self: Sized,
    {
        RangedPermutations::new(self, length)
    }
}

impl<T> Iterators for T where T: Iterator {}