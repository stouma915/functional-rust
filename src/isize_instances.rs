use crate::Semigroup;

impl Semigroup<isize> for isize {
    fn combine(x: isize, y: isize) -> isize {
        x + y
    }
}
