use crate::Nest::{self, Item, List};

/// Parse `Nest` recursively from string
///
/// No error handling!
///
/// Lists are delimited by square brackets `[` and `]`, and items are separated by commas `,`
///
/// # Examples
///
/// ```
/// # use nest::{parse, Nest};
/// let text = "a, [b, [c]], [d, e]";
///
/// // Parse with items as strings
/// let strings: Nest<String> = parse(text).unwrap();
///
/// assert_eq!(
///   format!("{:?}", strings),
///   r#"["a", ["b", ["c"]], ["d", "e"]]"#
/// );
/// ```
pub fn parse(text: &str) -> Result<Nest<String>, ()> {
    Ok(List(parse_component(text, 0)?.0))
}

/// Parse single component (branch) of `Nest` from slice of original string
///
/// No error handling!
/// 
/// `depth` is an arbitrary limit of recursion
fn parse_component(text: &str, depth: usize) -> Result<(Vec<Nest<String>>, usize), ()> {
    // Check depth limit
    assert!(depth < 100);

    // List building
    let mut list: Vec<Nest<String>> = Vec::new();
    let mut item_building = String::new();

    // Loop over characters
    // Enumeration is used for returning `i` variable inside loop (on ']' match arm)
    let mut chars = text.chars().enumerate();
    while let Some((i, ch)) = chars.next() {
        match ch {
            // Skip spaces and line breaks
            ' ' | '\n' => (),

            // Comma separates values
            ',' => {
                // Add current item to list, reset item
                if !item_building.is_empty() {
                    list.push(Item(item_building));
                    item_building = String::new();
                }
            }

            // Start new recurse
            '[' => {
                // Get rest of string, from current index
                // This can be optimized
                let rest = chars
                    .clone()
                    .map(|x| x.1.to_string())
                    .collect::<Vec<_>>()
                    .join("");

                // Recurse with same function, with `rest` string, increase depth
                let (item_branch, increase_index) = parse_component(&rest, depth + 1)?;

                // Add new branch to list
                list.push(List(item_branch));

                // Increase index of loop
                chars.nth(increase_index);
            }

            // End this recurse
            // This arm nearly mirrors the final statements of this function
            ']' => {
                // Add final item to list
                if !item_building.is_empty() {
                    list.push(Item(item_building));
                }

                // Return current list as item branch, and current index of iterated string slice
                return Ok((list, i));
            }

            // Add any other character to item building string
            _ => item_building.push(ch),
        }
    }

    // Add final item to list
    if !item_building.is_empty() {
        list.push(Item(item_building));
    }

    // Return final list, and dummy index
    // Index would be better as an `Option<usize>`
    Ok((list, 0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_works() {
        assert_eq!(format!("{:?}", parse("a, b").unwrap()), r#"["a", "b"]"#);

        assert_eq!(format!("{:?}", parse("[a, b]").unwrap()), r#"[["a", "b"]]"#);

        assert_eq!(
            format!("{:?}", parse("[a, [b], c], [], d").unwrap()),
            r#"[["a", ["b"], "c"], [], "d"]"#
        );

        assert_eq!(
            format!(
                "{:?}",
                parse("4, 9, 10, 0")
                    .unwrap()
                    .map(&|x| x.parse::<i32>().unwrap())
            ),
            "[4, 9, 10, 0]"
        );
    }
}
