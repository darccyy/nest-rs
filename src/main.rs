use nest::parse;

fn main() {
    println!("Hello world!");

    let text = "a, [b, [c]], [d, e]";

    let nest = parse(text).unwrap();

    println!("{:?}", nest);
}
