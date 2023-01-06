use crate::Equ;

impl Equ<i8> for i8 {
    fn eqv(x: i8, y: i8) -> bool {
        x == y
    }
}
