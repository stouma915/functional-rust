use crate::{Monoid, Semigroup};

impl Monoid<u128> for u128 {
    fn zero() -> u128 {
        0
    }
}

impl Semigroup<u128> for u128 {
    fn combine(x: u128, y: u128) -> u128 {
        x + y
    }
}
