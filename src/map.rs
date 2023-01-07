use crate::Nest::{self, Item, List};

impl<T> Nest<T> {
    /// Recursively walk through `Nest`, and map each item in place
    ///
    /// Similar to `map` method on `Iter` trait
    ///
    /// # Examples
    ///
    /// ```
    /// # use nest::{parse, Nest};
    /// // Parse with items as strings
    /// let strings: Nest<String> = parse("1, [2, [3]], [4, 5]").unwrap();
    ///
    /// // Map recursively to integers
    /// let ints: Nest<i32> = strings.map(&|x| x.parse().unwrap());
    ///
    /// assert_eq!(format!("{:?}", ints), "[1, [2, [3]], [4, 5]]");
    /// ```
    pub fn map<B, F>(self: Nest<T>, f: &F) -> Nest<B>
    where
        T: Sized,
        F: Fn(T) -> B,
    {
        match self {
            // Single item
            Item(item) => Item(f(item)),

            // Recurse
            List(vec) => {
                // This could be an `Iter` map, instead of a for loop
                let mut new = vec![];
                for item in vec {
                    new.push(item.map(f))
                }
                List(new)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_works() {
        // Explicitly typed - not parsed
        let nest = List(vec![
            List(vec![Item(1), List(vec![Item(2)]), Item(3)]),
            List(vec![]),
            Item(4),
        ]);

        // Add 10 to each item
        let mapped = nest.map(&|x| x + 10);

        assert_eq!(format!("{:?}", mapped), "[[11, [12], 13], [], 14]");
    }
}
