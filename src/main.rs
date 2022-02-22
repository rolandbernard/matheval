
use matheval::Expr;
use matheval::Number;
use matheval::Value;

fn main() {
    println!("{:?}", Number::read_from("1.01").unwrap().pow(&Number::read_from("-10").unwrap()));
    println!("{:?}", Expr::parse("5 + 5 (6 + 7) * 3 ^ (2 - 1) * atan2(-3, 4)").unwrap());
}

