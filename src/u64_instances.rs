use crate::Semigroup;

impl Semigroup<u64> for u64 {
    fn combine(x: u64, y: u64) -> u64 {
        x + y
    }
}
