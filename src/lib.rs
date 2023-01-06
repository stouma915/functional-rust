mod i128_instances;
mod i16_instances;
mod i32_instances;
mod i64_instances;
mod i8_instances;
mod isize_instances;
mod option_instances;
mod result_instances;
mod string_instances;
mod u128_instances;
mod u16_instances;
mod u32_instances;
mod u64_instances;
mod u8_instances;
mod usize_instances;
mod vec_instances;

pub trait Functor {
    type F<A>;

    fn fmap<A, B>(fa: Self::F<A>, op: fn(A) -> B) -> Self::F<B>;
}

pub trait Apply: Functor {
    fn apply<A: Clone, B>(fa: Self::F<A>, ff: Self::F<fn(A) -> B>) -> Self::F<B>;
}

pub trait Applicative: Apply {
    fn pure<A>(x: A) -> Self::F<A>;
}

pub trait Semigroup<A> {
    fn combine(x: A, y: A) -> A;
}
