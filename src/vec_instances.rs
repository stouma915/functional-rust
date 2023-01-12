use crate::{Applicative, Apply, Functor, Monoid, Semigroup};

impl<T> Applicative for Vec<T> {
    fn pure<A>(x: A) -> Self::F<A> {
        vec![x]
    }
}

impl<T> Apply for Vec<T> {
    fn apply<A: Clone, B, FN: FnOnce(A) -> B>(fa: Self::F<A>, ff: Self::F<FN>) -> Self::F<B> {
        let mut mapped: Vec<B> = vec![];

        for op in ff {
            mapped.append(&mut Self::fmap(fa.clone(), op));
        }

        mapped
    }
}

impl<T> Functor for Vec<T> {
    type F<A> = Vec<A>;

    fn fmap<A, B, FN: FnOnce(A) -> B + Copy>(fa: Self::F<A>, op: FN) -> Self::F<B> {
        let mut mapped = vec![];

        for a in fa {
            mapped.push(op(a));
        }

        mapped
    }
}

impl<T> Monoid<Vec<T>> for Vec<T> {
    fn zero() -> Vec<T> {
        vec![]
    }
}

impl<T> Semigroup<Vec<T>> for Vec<T> {
    fn combine(x: Vec<T>, y: Vec<T>) -> Vec<T> {
        let mut appended = vec![];

        for ex in x {
            appended.push(ex);
        }
        for ey in y {
            appended.push(ey);
        }

        appended
    }
}
