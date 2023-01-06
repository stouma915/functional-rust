use crate::{Equ, Functor};

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

impl<T> Functor for Option<T> {
    type F<A> = Option<A>;

    fn fmap<A, B>(fa: Self::F<A>, op: fn(&A) -> B) -> Self::F<B> {
        match fa {
            Some(a) => Some(op(&a)),
            None => None,
        }
    }
}
