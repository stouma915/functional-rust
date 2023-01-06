mod bool_instances;
mod char_instances;
mod i128_instances;
mod i16_instances;
mod i32_instances;
mod i64_instances;
mod i8_instances;
mod isize_instances;
mod option_instances;
mod result_instances;
mod str_instances;
mod string_instances;
mod u128_instances;
mod u16_instances;
mod u32_instances;
mod u64_instances;
mod u8_instances;
mod usize_instances;
mod vec_instances;

pub trait Equ<A> {
    fn eqv(x: A, y: A) -> bool;
    fn n_eqv(x: A, y: A) -> bool {
        !Self::eqv(x, y)
    }
}
