use crate::Semigroup;

impl Semigroup<u128> for u128 {
    fn combine(x: u128, y: u128) -> u128 {
        x + y
    }
}
