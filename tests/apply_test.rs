extern crate functional_rust;

use functional_rust::Apply;

#[test]
fn t_option() {
    assert_eq!(Option::<i8>::apply(Some(1), Some(|x| x + 1)), Some(2));
    assert_eq!(Option::<i8>::apply(None, Some(|x: i8| x + 1)), None);
    assert_eq!(Option::<i8>::apply(Some(1), None as Option<i8>), None as Option<i8>);
}

#[test]
fn t_vec() {
    assert_eq!(
        Vec::<i8>::apply(vec!(1, 2), vec!(|x| x + 1, |x| x + 2)),
        vec!(2, 3, 3, 4)
    );
    assert_eq!(
        Vec::<i8>::apply(vec!(1, 2, 3), vec!(|x| x + 1, |x| x + 2)),
        vec!(2, 3, 4, 3, 4, 5)
    );
    assert_eq!(
        Vec::<i8>::apply(vec!(1, 2), vec!(|x| x + 1, |x| x + 2, |x| x + 3)),
        vec!(2, 3, 3, 4, 4, 5)
    );
}
