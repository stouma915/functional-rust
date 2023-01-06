use crate::Functor;

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
