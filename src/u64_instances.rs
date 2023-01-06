use crate::Equ;

impl Equ<u64> for u64 {
    fn eqv(x: u64, y: u64) -> bool {
        x == y
    }
}
