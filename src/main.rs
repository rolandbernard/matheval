
use matheval::Expr;
use matheval::Number;

fn main() {
    let expr = Expr::parse("0.2").unwrap();
    let res = expr.eval::<Number>().unwrap();
    println!("{:?}", res);
}

