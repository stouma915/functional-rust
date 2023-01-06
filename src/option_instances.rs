use crate::Functor;

impl<T> Functor for Option<T> {
    type F<A> = Option<A>;

    fn fmap<A, B>(fa: Self::F<A>, op: fn(A) -> B) -> Self::F<B> {
        match fa {
            Some(a) => Some(op(a)),
            None => None,
        }
    }
}
