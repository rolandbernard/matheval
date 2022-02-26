
use std::str::FromStr;

use matheval::Expr;
use matheval::Number;
use matheval::NumberContext;

fn main() {
    let expr = Expr::parse("-5 * 6 * 7").unwrap();
    println!("{:?}", expr);
    println!("{:?}", expr.to_string());
    let res = expr.eval_in(&NumberContext::new()).unwrap();
    println!("{:?}", res);
    println!("{:?}", res.to_f64());
    println!("{:?}", res.to_string());
    println!("{:?}", Number::from_str("0b1e11"));
}

