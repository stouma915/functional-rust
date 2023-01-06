use crate::Equ;

impl Equ<u16> for u16 {
    fn eqv(x: u16, y: u16) -> bool {
        x == y
    }
}
