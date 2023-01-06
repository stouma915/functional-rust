use crate::{Monoid, Semigroup};

impl Monoid<usize> for usize {
    fn zero() -> usize {
        0
    }
}

impl Semigroup<usize> for usize {
    fn combine(x: usize, y: usize) -> usize {
        x + y
    }
}
