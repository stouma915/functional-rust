use crate::Equ;

impl Equ<u8> for u8 {
    fn eqv(x: u8, y: u8) -> bool {
        x == y
    }
}
