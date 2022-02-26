
use matheval::Expr;
use matheval::Number;
use matheval::NumberContext;
use matheval::Value;

fn main() {
    let expr = Expr::parse("1.1e+1 * 100 + 7 / 100").unwrap();
    println!("{:?}", expr);
    let res = expr.eval_in(&NumberContext::new()).unwrap();
    println!("{:?}", res);
    println!("{:?}", Number::parse_from("0b1e11"));
}

