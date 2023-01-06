use crate::Semigroup;

impl Semigroup<i128> for i128 {
    fn combine(x: i128, y: i128) -> i128 {
        x + y
    }
}
