use crate::{Applicative, Apply, Functor};

impl<T> Applicative for Vec<T> {
    fn pure<A>(x: A) -> Self::F<A> {
        vec![x]
    }
}

impl<T> Apply for Vec<T> {
    fn apply<A: Clone, B>(fa: Self::F<A>, ff: Self::F<fn(A) -> B>) -> Self::F<B> {
        let mut mapped: Vec<B> = vec![];

        for op in ff {
            mapped.append(&mut Self::fmap(fa.clone(), op));
        }

        mapped
    }
}

impl<T> Functor for Vec<T> {
    type F<A> = Vec<A>;

    fn fmap<A, B>(fa: Self::F<A>, op: fn(A) -> B) -> Self::F<B> {
        let mut mapped = vec![];

        for a in fa {
            mapped.push(op(a));
        }

        mapped
    }
}
