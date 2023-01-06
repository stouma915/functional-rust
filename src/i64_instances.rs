use crate::Semigroup;

impl Semigroup<i64> for i64 {
    fn combine(x: i64, y: i64) -> i64 {
        x + y
    }
}
