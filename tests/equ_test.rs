extern crate functional_rust;

use functional_rust::Equ;

#[test]
fn t_bool() {
    assert_eq!(bool::eqv(true, true), true);
    assert_eq!(bool::eqv(true, false), false);
    assert_eq!(bool::n_eqv(true, true), false);
    assert_eq!(bool::n_eqv(true, false), true);
}

#[test]
fn t_char() {
    assert_eq!(char::eqv('A', 'A'), true);
    assert_eq!(char::eqv('A', 'B'), false);
    assert_eq!(char::n_eqv('A', 'A'), false);
    assert_eq!(char::n_eqv('A', 'B'), true);
}

#[test]
fn t_i8() {
    assert_eq!(i8::eqv(1, 1), true);
    assert_eq!(i8::eqv(1, 2), false);
    assert_eq!(i8::n_eqv(1, 1), false);
    assert_eq!(i8::n_eqv(1, 2), true);
}

#[test]
fn t_i16() {
    assert_eq!(i16::eqv(1, 1), true);
    assert_eq!(i16::eqv(1, 2), false);
    assert_eq!(i16::n_eqv(1, 1), false);
    assert_eq!(i16::n_eqv(1, 2), true);
}

#[test]
fn t_i32() {
    assert_eq!(i32::eqv(1, 1), true);
    assert_eq!(i32::eqv(1, 2), false);
    assert_eq!(i32::n_eqv(1, 1), false);
    assert_eq!(i32::n_eqv(1, 2), true);
}

#[test]
fn t_i64() {
    assert_eq!(i64::eqv(1, 1), true);
    assert_eq!(i64::eqv(1, 2), false);
    assert_eq!(i64::n_eqv(1, 1), false);
    assert_eq!(i64::n_eqv(1, 2), true);
}

#[test]
fn t_i128() {
    assert_eq!(i128::eqv(1, 1), true);
    assert_eq!(i128::eqv(1, 2), false);
    assert_eq!(i128::n_eqv(1, 1), false);
    assert_eq!(i128::n_eqv(1, 2), true);
}

#[test]
fn t_isize() {
    assert_eq!(isize::eqv(1, 1), true);
    assert_eq!(isize::eqv(1, 2), false);
    assert_eq!(isize::n_eqv(1, 1), false);
    assert_eq!(isize::n_eqv(1, 2), true);
}

#[test]
fn t_option() {
    assert_eq!(Option::<i8>::eqv(Some(1), Some(1)), true);
    assert_eq!(Option::<i8>::eqv(Some(1), Some(2)), false);
    assert_eq!(Option::<i8>::eqv(Some(1), None), false);
    assert_eq!(Option::<i8>::eqv(None, None), true);
    assert_eq!(Option::<i8>::n_eqv(Some(1), Some(1)), false);
    assert_eq!(Option::<i8>::n_eqv(Some(1), Some(2)), true);
    assert_eq!(Option::<i8>::n_eqv(Some(1), None), true);
    assert_eq!(Option::<i8>::n_eqv(None, None), false);
}

#[test]
fn t_result() {
    assert_eq!(Result::<i8, &str>::eqv(Ok(1), Ok(1)), true);
    assert_eq!(Result::<i8, &str>::eqv(Ok(1), Ok(2)), false);
    assert_eq!(Result::<i8, &str>::eqv(Err("A"), Err("A")), true);
    assert_eq!(Result::<i8, &str>::eqv(Err("A"), Err("B")), false);
    assert_eq!(Result::<i8, &str>::eqv(Ok(1), Err("a")), false);
    assert_eq!(Result::<i8, &str>::n_eqv(Ok(1), Ok(1)), false);
    assert_eq!(Result::<i8, &str>::n_eqv(Ok(1), Ok(2)), true);
    assert_eq!(Result::<i8, &str>::n_eqv(Err("A"), Err("A")), false);
    assert_eq!(Result::<i8, &str>::n_eqv(Err("A"), Err("B")), true);
    assert_eq!(Result::<i8, &str>::n_eqv(Ok(1), Err("a")), true);
}

#[test]
fn t_str() {
    assert_eq!(str::eqv("ABC", "ABC"), true);
    assert_eq!(str::eqv("ABC", "123"), false);
    assert_eq!(str::n_eqv("ABC", "ABC"), false);
    assert_eq!(str::n_eqv("ABC", "123"), true);
}

#[test]
fn t_string() {
    assert_eq!(String::eqv(String::from("ABC"), String::from("ABC")), true);
    assert_eq!(String::eqv(String::from("ABC"), String::from("123")), false);
    assert_eq!(
        String::n_eqv(String::from("ABC"), String::from("ABC")),
        false
    );
    assert_eq!(
        String::n_eqv(String::from("ABC"), String::from("123")),
        true
    );
}

#[test]
fn t_u8() {
    assert_eq!(u8::eqv(1, 1), true);
    assert_eq!(u8::eqv(1, 2), false);
    assert_eq!(u8::n_eqv(1, 1), false);
    assert_eq!(u8::n_eqv(1, 2), true);
}

#[test]
fn t_u16() {
    assert_eq!(u16::eqv(1, 1), true);
    assert_eq!(u16::eqv(1, 2), false);
    assert_eq!(u16::n_eqv(1, 1), false);
    assert_eq!(u16::n_eqv(1, 2), true);
}

#[test]
fn t_u32() {
    assert_eq!(u32::eqv(1, 1), true);
    assert_eq!(u32::eqv(1, 2), false);
    assert_eq!(u32::n_eqv(1, 1), false);
    assert_eq!(u32::n_eqv(1, 2), true);
}

#[test]
fn t_u64() {
    assert_eq!(u64::eqv(1, 1), true);
    assert_eq!(u64::eqv(1, 2), false);
    assert_eq!(u64::n_eqv(1, 1), false);
    assert_eq!(u64::n_eqv(1, 2), true);
}

#[test]
fn t_u128() {
    assert_eq!(u128::eqv(1, 1), true);
    assert_eq!(u128::eqv(1, 2), false);
    assert_eq!(u128::n_eqv(1, 1), false);
    assert_eq!(u128::n_eqv(1, 2), true);
}

#[test]
fn t_usize() {
    assert_eq!(usize::eqv(1, 1), true);
    assert_eq!(usize::eqv(1, 2), false);
    assert_eq!(usize::n_eqv(1, 1), false);
    assert_eq!(usize::n_eqv(1, 2), true);
}

#[test]
fn t_vec() {
    assert_eq!(Vec::<i32>::eqv(vec!(1, 2, 3), vec!(1, 2, 3)), true);
    assert_eq!(Vec::<i32>::eqv(vec!(1, 2, 3), vec!(4, 5, 6)), false);
    assert_eq!(Vec::<i32>::eqv(vec!(1, 2, 3), vec!()), false);
    assert_eq!(Vec::<i32>::eqv(vec!(), vec!()), true);
    assert_eq!(Vec::<i32>::n_eqv(vec!(1, 2, 3), vec!(1, 2, 3)), false);
    assert_eq!(Vec::<i32>::n_eqv(vec!(1, 2, 3), vec!(4, 5, 6)), true);
    assert_eq!(Vec::<i32>::n_eqv(vec!(1, 2, 3), vec!()), true);
    assert_eq!(Vec::<i32>::n_eqv(vec!(), vec!()), false);
}
