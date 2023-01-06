use crate::Semigroup;

impl<A, B> Semigroup<Result<A, B>> for Result<A, B> {
    fn combine(x: Result<A, B>, y: Result<A, B>) -> Result<A, B> {
        match x {
            Ok(_) => x,
            Err(_) => y,
        }
    }
}
