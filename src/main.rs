
use matheval::Expr;
use matheval::Number;

fn main() {
    let expr = Expr::parse("5 + 5 (6 + 7) * 3 ^ (2 - 1) / (2 ^ 2) ^ 3").unwrap();
    let res = expr.eval::<Number>();
    println!("{:?}", res);
}

