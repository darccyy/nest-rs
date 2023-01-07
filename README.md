# Recursive Vector Nesting Example

Just an example

```rs
use nest::{parse, Nest};

fn main() {
    println!("Hello world!");

    let text = "1, [2, [3]], [4, 5]";

    let strings: Nest<String> = parse(text).unwrap();
    assert_eq!(
        format!("{:?}", strings),
        r#"["1", ["2", ["3"]], ["4", "5"]]"#
    );

    let ints: Nest<i32> = strings.map(&|x| x.parse().unwrap());
    assert_eq!(
      format!("{:?}", ints),
      "[1, [2, [3]], [4, 5]]"
    );
}
```
