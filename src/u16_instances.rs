use crate::{Monoid, Semigroup};

impl Monoid<u16> for u16 {
    fn zero() -> u16 {
        0
    }
}

impl Semigroup<u16> for u16 {
    fn combine(x: u16, y: u16) -> u16 {
        x + y
    }
}
