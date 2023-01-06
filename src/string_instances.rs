use crate::Semigroup;

impl Semigroup<String> for String {
    fn combine(x: String, y: String) -> String {
        String::from_iter(vec![x, y])
    }
}
