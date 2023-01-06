use crate::{Monoid, Semigroup};

impl Monoid<u32> for u32 {
    fn zero() -> u32 {
        0
    }
}

impl Semigroup<u32> for u32 {
    fn combine(x: u32, y: u32) -> u32 {
        x + y
    }
}
