use crate::{Monoid, Semigroup};

impl Monoid<i8> for i8 {
    fn zero() -> i8 {
        0
    }
}

impl Semigroup<i8> for i8 {
    fn combine(x: i8, y: i8) -> i8 {
        x + y
    }
}
