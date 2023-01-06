use crate::Semigroup;

impl Semigroup<u16> for u16 {
    fn combine(x: u16, y: u16) -> u16 {
        x + y
    }
}
