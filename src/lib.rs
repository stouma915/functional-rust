mod option_instances;
mod vec_instances;

pub trait Functor {
    type F<A>;

    fn fmap<A, B>(fa: Self::F<A>, op: fn(A) -> B) -> Self::F<B>;
}

pub trait Apply: Functor {
    fn apply<A: Clone, B>(fa: Self::F<A>, ff: Self::F<fn(A) -> B>) -> Self::F<B>;
}
