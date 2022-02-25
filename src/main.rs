
use matheval::Expr;
use matheval::Number;
use matheval::Value;

fn main() {
    let expr = Expr::parse("0.2 + 0.1").unwrap();
    println!("{:?}", expr);
    let res = expr.eval::<Number>().unwrap();
    println!("{:?}", res);
    println!("{:?}", Number::parse_from("0.2"));
}

