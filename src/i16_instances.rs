use crate::{Monoid, Semigroup};

impl Monoid<i16> for i16 {
    fn zero() -> i16 {
        0
    }
}

impl Semigroup<i16> for i16 {
    fn combine(x: i16, y: i16) -> i16 {
        x + y
    }
}
