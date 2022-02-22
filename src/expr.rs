
pub mod value;
pub mod parse;
pub mod evaluate;
pub mod simplify;

#[derive(PartialEq)]
pub enum Expr {
    Constant(value::Value),
    Sum(Vec<Expr>),
    Product(Vec<Expr>),
    Power(Vec<Expr>),
    Function(String, Box<Expr>),
    Variable(String),
}

impl Expr {
    pub fn negate(some: Expr) -> Expr {
        Expr::Product(vec![Expr::Constant(value::Value::Rational(-1, 1)), some])
    }

    pub fn reciprocal(some: Expr) -> Expr {
        Expr::Power(vec![some, Expr::Constant(value::Value::Rational(-1, 1))])
    }
}

