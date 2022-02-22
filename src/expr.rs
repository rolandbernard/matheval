
mod value;
mod evaluate;
mod simplify;
mod parse;
mod format;

pub use value::Value;

#[derive(PartialEq, Debug)]
pub enum Expr {
    Constant(Value),
    Sum(Vec<Expr>),
    Product(Vec<Expr>),
    Power(Vec<Expr>),
    Function(String, Box<Expr>),
    Variable(String),
}

impl Expr {
    fn negate(some: Expr) -> Expr {
        Expr::Product(vec![Expr::Constant(Value::Rational(-1, 1)), some])
    }

    fn reciprocal(some: Expr) -> Expr {
        Expr::Power(vec![some, Expr::Constant(Value::Rational(-1, 1))])
    }

    pub fn parse(source: &str) -> Result<Expr, parse::ParseError> {
        parse::parse(source)
    }
}

