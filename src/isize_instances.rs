use crate::{Monoid, Semigroup};

impl Monoid<isize> for isize {
    fn zero() -> isize {
        0
    }
}

impl Semigroup<isize> for isize {
    fn combine(x: isize, y: isize) -> isize {
        x + y
    }
}
