use crate::Equ;

impl<A: PartialEq, B: PartialEq> Equ<Result<A, B>> for Result<A, B> {
    fn eqv(x: Result<A, B>, y: Result<A, B>) -> bool {
        match x {
            Ok(a) => match y {
                Ok(b) => a == b,
                Err(_) => false,
            },
            Err(a) => match y {
                Ok(_) => false,
                Err(b) => a == b,
            },
        }
    }
}
