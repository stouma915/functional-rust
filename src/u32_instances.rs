use crate::Semigroup;

impl Semigroup<u32> for u32 {
    fn combine(x: u32, y: u32) -> u32 {
        x + y
    }
}
