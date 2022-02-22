
mod value;
mod evaluate;
mod simplify;
mod parser;
mod format;

pub use value::Value;
pub use value::EvalError;
pub use parser::ParseError;

#[derive(PartialEq, Debug)]
pub enum Expr {
    Constant(String),
    Sum(Vec<Expr>),
    Product(Vec<Expr>),
    Power(Vec<Expr>),
    Function(String, Vec<Expr>),
    Variable(String),
}

impl Expr {
    fn negate(some: Expr) -> Expr {
        Expr::Product(vec![Expr::Constant("-1".to_owned()), some])
    }

    fn reciprocal(some: Expr) -> Expr {
        Expr::Power(vec![some, Expr::Constant("-1".to_owned())])
    }

    pub fn parse(source: &str) -> Result<Expr, parser::ParseError> {
        parser::parse(source)
    }
}

