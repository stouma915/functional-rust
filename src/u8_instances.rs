use crate::{Monoid, Semigroup};

impl Monoid<u8> for u8 {
    fn zero() -> u8 {
        0
    }
}

impl Semigroup<u8> for u8 {
    fn combine(x: u8, y: u8) -> u8 {
        x + y
    }
}
