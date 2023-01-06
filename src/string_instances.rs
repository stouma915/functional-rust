use crate::Equ;

impl Equ<String> for String {
    fn eqv(x: String, y: String) -> bool {
        x == y
    }
}
