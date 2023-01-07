mod parse;

use std::fmt;

pub use parse::parse;
use Nest::{End, More};

pub enum Nest<T> {
    End(T),
    More(Vec<Nest<T>>),
}

impl<T> fmt::Debug for Nest<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            End(x) => write!(f, "{:?}", x),
            More(x) => write!(f, "{:?}", x),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nest_works() {
        let nest = More(vec![
            More(vec![End(1), More(vec![End(2)]), End(3)]),
            More(vec![]),
            End(4),
        ]);

        assert_eq!(format!("{:?}", nest), "[[1, [2], 3], [], 4]");
    }
}
