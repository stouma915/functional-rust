use crate::Semigroup;

impl Semigroup<i8> for i8 {
    fn combine(x: i8, y: i8) -> i8 {
        x + y
    }
}
