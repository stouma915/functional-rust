mod option_instances;
mod vec_instances;

pub trait Functor {
    type F<A>;

    fn fmap<A, B>(fa: Self::F<A>, op: fn(A) -> B) -> Self::F<B>;
}
