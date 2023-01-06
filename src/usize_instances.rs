use crate::Semigroup;

impl Semigroup<usize> for usize {
    fn combine(x: usize, y: usize) -> usize {
        x + y
    }
}
