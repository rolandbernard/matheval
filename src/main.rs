
use matheval::parse::parse;

fn main() {
    println!("{:?}", parse("5 + 5 * 3 ^ (2 - 1)"));
}

