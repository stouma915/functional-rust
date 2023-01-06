use crate::Equ;

impl Equ<u32> for u32 {
    fn eqv(x: u32, y: u32) -> bool {
        x == y
    }
}
