
use matheval::Expr;

fn main() {
    println!("{:?}", Expr::parse("5 + 5 (6 + 7) * 3 ^ (2 - 1) * atan2(-3, 4)").unwrap());
}

