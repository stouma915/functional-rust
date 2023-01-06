use crate::{Monoid, Semigroup};

impl Monoid<String> for String {
    fn zero() -> String {
        String::from("")
    }
}

impl Semigroup<String> for String {
    fn combine(x: String, y: String) -> String {
        String::from_iter(vec![x, y])
    }
}
