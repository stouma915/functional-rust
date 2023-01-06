use crate::Equ;

impl Equ<bool> for bool {
    fn eqv(x: bool, y: bool) -> bool {
        x == y
    }
}
