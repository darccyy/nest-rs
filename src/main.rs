use nest::{parse, Nest};

fn main() {
    // Example text
    // Note: No top-level brackets
    let text = "1, [2, [3]], [4, 5]";

    // Parse as strings
    let strings: Nest<String> = parse(text).unwrap();
    assert_eq!(
        format!("{:?}", strings),
        r#"["1", ["2", ["3"]], ["4", "5"]]"#
    );

    // Map recursively to integers
    let ints: Nest<i32> = strings.map(&|x| x.parse().unwrap());
    assert_eq!(format!("{:?}", ints), "[1, [2, [3]], [4, 5]]");
}
