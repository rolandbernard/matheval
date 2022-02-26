
use std::str::FromStr;

use matheval::Expr;
use matheval::Number;
use matheval::NumberContext;

fn main() {
    let expr = Expr::parse("1.1e+1 * 100 + 7 / 100 - 6 ^ -2 / 8 ^ 2 * pi + e").unwrap();
    println!("{:?}", expr);
    let res = expr.eval_in(&NumberContext::new()).unwrap();
    println!("{:?}", res);
    println!("{:?}", res.to_f64());
    println!("{:?}", res.to_string());
    println!("{:?}", Number::from_str("0b1e11"));
}

