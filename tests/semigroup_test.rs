extern crate functional_rust;

use functional_rust::Semigroup;

#[test]
fn t_i8() {
    assert_eq!(i8::combine(1, 1), 2);
}

#[test]
fn t_i16() {
    assert_eq!(i16::combine(1, 1), 2);
}

#[test]
fn t_i32() {
    assert_eq!(i32::combine(1, 1), 2);
}

#[test]
fn t_i64() {
    assert_eq!(i64::combine(1, 1), 2);
}

#[test]
fn t_i128() {
    assert_eq!(i128::combine(1, 1), 2);
}

#[test]
fn t_isize() {
    assert_eq!(isize::combine(1, 1), 2);
}

#[test]
fn t_option() {
    assert_eq!(Option::<i8>::combine(Some(1), Some(1)), Some(2));
    assert_eq!(Option::<i8>::combine(None, Some(1)), Some(1));
    assert_eq!(Option::<i8>::combine(Some(1), None), Some(1));
    assert_eq!(Option::<i8>::combine(None, None), None);
}

#[test]
fn t_result() {
    assert_eq!(Result::<i8, i8>::combine(Ok(1), Ok(2)), Ok(1));
    assert_eq!(Result::<i8, i8>::combine(Err(0), Ok(1)), Ok(1));
    assert_eq!(Result::<i8, i8>::combine(Ok(1), Err(0)), Ok(1));
    assert_eq!(Result::<i8, i8>::combine(Err(1), Err(0)), Err(0));
}

#[test]
fn t_string() {
    assert_eq!(
        String::combine(String::from("A"), String::from("B")),
        String::from("AB")
    );
}

#[test]
fn t_u8() {
    assert_eq!(u8::combine(1, 1), 2);
}

#[test]
fn t_u16() {
    assert_eq!(u16::combine(1, 1), 2);
}

#[test]
fn t_u32() {
    assert_eq!(u32::combine(1, 1), 2);
}

#[test]
fn t_u64() {
    assert_eq!(u64::combine(1, 1), 2);
}

#[test]
fn t_u128() {
    assert_eq!(u128::combine(1, 1), 2);
}

#[test]
fn t_usize() {
    assert_eq!(usize::combine(1, 1), 2);
}

#[test]
fn t_vec() {
    assert_eq!(Vec::<i8>::combine(vec![1, 2], vec![3, 4]), vec![1, 2, 3, 4]);
}
