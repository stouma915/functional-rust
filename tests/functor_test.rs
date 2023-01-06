extern crate functional_rust;

use functional_rust::{Equ, Functor};

#[test]
fn t_option() {
    assert_eq!(
        Option::<i8>::eqv(Option::<i8>::fmap(Some(1), |x| x + 1), Some(2)),
        true
    );
    assert_eq!(
        Option::<i8>::eqv(Option::<i8>::fmap(None as Option<i8>, |x| x + 1), None),
        true
    );
}

#[test]
fn t_vec() {
    assert_eq!(
        Vec::<i8>::eqv(Vec::<i8>::fmap(vec!(1, 2, 3), |x| x + 1), vec!(2, 3, 4)),
        true
    );
    assert_eq!(
        Vec::<i8>::eqv(Vec::<i8>::fmap(vec!() as Vec<i8>, |x| x + 1), vec!()),
        true
    );
}
