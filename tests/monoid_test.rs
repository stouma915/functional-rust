extern crate functional_rust;

use functional_rust::Monoid;

#[test]
fn t_i8() {
    assert_eq!(i8::zero(), 0);
}

#[test]
fn t_i16() {
    assert_eq!(i16::zero(), 0);
}

#[test]
fn t_i32() {
    assert_eq!(i32::zero(), 0);
}

#[test]
fn t_i64() {
    assert_eq!(i64::zero(), 0);
}

#[test]
fn t_i128() {
    assert_eq!(i128::zero(), 0);
}

#[test]
fn t_isize() {
    assert_eq!(isize::zero(), 0);
}

#[test]
fn t_option() {
    assert_eq!(Option::<i8>::zero(), None);
}

#[test]
fn t_string() {
    assert_eq!(String::zero(), String::from(""));
}

#[test]
fn t_u8() {
    assert_eq!(u8::zero(), 0);
}

#[test]
fn t_u16() {
    assert_eq!(u16::zero(), 0);
}

#[test]
fn t_u32() {
    assert_eq!(u32::zero(), 0);
}

#[test]
fn t_u64() {
    assert_eq!(u64::zero(), 0);
}

#[test]
fn t_u128() {
    assert_eq!(u128::zero(), 0);
}

#[test]
fn t_usize() {
    assert_eq!(usize::zero(), 0);
}

#[test]
fn t_vec() {
    assert_eq!(Vec::<i8>::zero(), vec![]);
}
