use crate::{Applicative, Apply, Functor, Semigroup};

impl<T> Applicative for Option<T> {
    fn pure<A>(x: A) -> Self::F<A> {
        Some(x)
    }
}

impl<T> Apply for Option<T> {
    fn apply<A, B>(fa: Self::F<A>, ff: Self::F<fn(A) -> B>) -> Self::F<B> {
        match ff {
            Some(op) => Self::fmap(fa, op),
            None => None,
        }
    }
}

impl<T> Functor for Option<T> {
    type F<A> = Option<A>;

    fn fmap<A, B>(fa: Self::F<A>, op: fn(A) -> B) -> Self::F<B> {
        match fa {
            Some(a) => Some(op(a)),
            None => None,
        }
    }
}

impl<T: Semigroup<T> + Copy> Semigroup<Option<T>> for Option<T> {
    fn combine(x: Option<T>, y: Option<T>) -> Option<T> {
        match x {
            Some(a) => match y {
                Some(b) => Some(T::combine(a, b)),
                None => x.clone(),
            },
            None => y,
        }
    }
}
