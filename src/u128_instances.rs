use crate::Equ;

impl Equ<u128> for u128 {
    fn eqv(x: u128, y: u128) -> bool {
        x == y
    }
}
