use crate::Equ;

impl Equ<char> for char {
    fn eqv(x: char, y: char) -> bool {
        x == y
    }
}
