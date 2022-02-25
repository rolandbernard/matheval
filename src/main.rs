
use matheval::Expr;
use matheval::Number;
use matheval::NumberContext;
use matheval::Value;

fn main() {
    let expr = Expr::parse("0b1.1").unwrap();
    println!("{:?}", expr);
    let res = expr.eval_in(&NumberContext::new()).unwrap();
    println!("{:?}", res);
    println!("{:?}", Number::parse_from("0.2"));
}

