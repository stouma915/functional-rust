use crate::{Monoid, Semigroup};

impl Monoid<i128> for i128 {
    fn zero() -> i128 {
        0
    }
}

impl Semigroup<i128> for i128 {
    fn combine(x: i128, y: i128) -> i128 {
        x + y
    }
}
