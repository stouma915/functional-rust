use crate::Equ;

impl Equ<i16> for i16 {
    fn eqv(x: i16, y: i16) -> bool {
        x == y
    }
}
