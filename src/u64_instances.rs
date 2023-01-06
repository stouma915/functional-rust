use crate::{Monoid, Semigroup};

impl Monoid<u64> for u64 {
    fn zero() -> u64 {
        0
    }
}

impl Semigroup<u64> for u64 {
    fn combine(x: u64, y: u64) -> u64 {
        x + y
    }
}
