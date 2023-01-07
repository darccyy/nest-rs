mod map;
mod parse;

pub use parse::parse;

use std::fmt;

use Nest::{Item, List};

/// Nested list
///
/// Generic type `T` must have `Sized` trait to use `map` method
pub enum Nest<T> {
    /// Single item
    Item(T),
    /// New list
    List(Vec<Nest<T>>),
}

// Custom debug implementation, removes `Item(...)` and `List(...)` syntax
impl<T> fmt::Debug for Nest<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Item(x) => write!(f, "{:?}", x),
            List(x) => write!(f, "{:?}", x),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nest_works() {
        // Explicitly typed - not parsed
        let nest = List(vec![
            List(vec![Item(1), List(vec![Item(2)]), Item(3)]),
            List(vec![]),
            Item(4),
        ]);

        assert_eq!(format!("{:?}", nest), "[[1, [2], 3], [], 4]");
    }
}
