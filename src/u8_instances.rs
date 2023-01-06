use crate::Semigroup;

impl Semigroup<u8> for u8 {
    fn combine(x: u8, y: u8) -> u8 {
        x + y
    }
}
