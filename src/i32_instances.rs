use crate::Equ;

impl Equ<i32> for i32 {
    fn eqv(x: i32, y: i32) -> bool {
        x == y
    }
}
