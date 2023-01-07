use crate::Nest::{self, End, More};

pub fn parse(text: &str) -> Result<Nest<String>, String> {
    Ok(More(parse_component(text, 0)?.0))
}

fn parse_component(text: &str, iter: usize) -> Result<(Vec<Nest<String>>, usize), String> {
    assert!(iter < 20);

    let mut list: Vec<Nest<String>> = Vec::new();
    let mut build = String::new();

    let mut chars = text.chars().enumerate();
    while let Some((i, ch)) = chars.next() {
        match ch {
            ' ' | '\n' => (),

            ',' => {
                if !build.is_empty() {
                    list.push(End(build));
                    build = String::new();
                }
            }

            '[' => {
                let rest = chars
                    .clone()
                    .map(|x| x.1.to_string())
                    .collect::<Vec<_>>()
                    .join("");

                let (branch, inc) = parse_component(&rest, iter + 1)?;

                list.push(More(branch));

                chars.nth(inc);
            }

            ']' => {
                if !build.is_empty() {
                    list.push(End(build));
                }

                return Ok((list, i));
            }

            _ => build.push(ch),
        }
    }

    if !build.is_empty() {
        list.push(End(build));
    }

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
