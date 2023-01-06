use crate::Equ;

impl<T: PartialEq> Equ<Option<T>> for Option<T> {
    fn eqv(x: Option<T>, y: Option<T>) -> bool {
        match x {
            Some(a) => match y {
                Some(b) => a == b,
                None => false,
            },
            None => y.is_none(),
        }
    }
}
