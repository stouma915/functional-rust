use crate::Equ;

impl Equ<isize> for isize {
    fn eqv(x: isize, y: isize) -> bool {
        x == y
    }
}
