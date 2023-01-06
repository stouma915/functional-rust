extern crate functional_rust;

use functional_rust::Applicative;

#[test]
fn t_option() {
    assert_eq!(Option::<i8>::pure(1), Some(1));
}

#[test]
fn t_vec() {
    assert_eq!(Vec::<i8>::pure(1), vec!(1));
}
