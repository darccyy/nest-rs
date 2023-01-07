use crate::Nest::{self, End, More};

pub fn parse(text: &str) -> Result<Nest<String>, String> {
    Ok(Nest::More(parse_component(text, 0)?.0))
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

fn _highlight(ch: char) -> String {
    format!(
        "\x1b[32{}\x1b[0m",
        match ch {
            ' ' => ";2m•".to_string(),
            '\n' => ";33m•".to_string(),
            ',' => ";34m,".to_string(),
            '[' => ";35m,".to_string(),
            ']' => ";36m,".to_string(),
            _ => format!("m{}", ch),
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_works() {
        let text = "[[a, [b], c], [], d]";
        let (nest, _) = parse_component(text, 0).unwrap();
        assert_eq!(
            format!("{:?}", nest.first().unwrap()),
            r#"[["a", ["b"], "c"], [], "d"]"#
        );

        let text = "a, b";
        let (nest, _) = parse_component(text, 0).unwrap();
        assert_eq!(
            format!("{:?}", nest.first().unwrap()),
            r#"[["a", ["b"], "c"], [], "d"]"#
        );
    }
}
