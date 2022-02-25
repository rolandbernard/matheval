
use matheval::Expr;
use matheval::Number;
use matheval::NumberContext;
use matheval::Value;

fn main() {
    let expr = Expr::parse("0.2 + 0.1^2^2 * 3 * (1 - 7 * 0.7) + sin(pi / 7)").unwrap();
    println!("{:?}", expr);
    let res = expr.eval_in(&NumberContext::new()).unwrap();
    println!("{:?}", res);
    println!("{:?}", Number::parse_from("0.2"));
}

