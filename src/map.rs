use crate::Nest::{self, End, More};

impl<T> Nest<T> {
    pub fn map<B, F>(self: Nest<T>, f: &F) -> Nest<B>
    where
        T: Sized,
        F: Fn(T) -> B,
    {
        match self {
            End(item) => End(f(item)),

            More(vec) => {
                let mut new = vec![];
                for item in vec {
                    new.push(item.map(f))
                }
                More(new)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_works() {
        let nest = More(vec![
            More(vec![End(1), More(vec![End(2)]), End(3)]),
            More(vec![]),
            End(4),
        ]);

        let mapped = nest.map(&|x| x + 10);

        assert_eq!(format!("{:?}", mapped), "[[11, [12], 13], [], 14]");
    }
}
