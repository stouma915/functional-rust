use crate::Equ;

impl Equ<usize> for usize {
    fn eqv(x: usize, y: usize) -> bool {
        x == y
    }
}
