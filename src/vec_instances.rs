use crate::Equ;

impl<T: PartialEq> Equ<Vec<T>> for Vec<T> {
    fn eqv(x: Vec<T>, y: Vec<T>) -> bool {
        if usize::eqv(x.len(), y.len()) {
            !x.iter()
                .zip(y.iter())
                .map(|(e_x, e_y)| e_x == e_y)
                .collect::<Vec<bool>>()
                .contains(&false)
        } else {
            false
        }
    }
}
