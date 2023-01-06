use crate::Equ;

impl Equ<&str> for str {
    fn eqv(x: &str, y: &str) -> bool {
        x == y
    }
}
