use crate::Equ;

impl Equ<i64> for i64 {
    fn eqv(x: i64, y: i64) -> bool {
        x == y
    }
}
