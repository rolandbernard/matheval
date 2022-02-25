
mod value;
mod evaluate;
mod simplify;
mod parser;
mod format;

pub use value::Value;
pub use value::Context;
pub use value::ContextFn;
pub use evaluate::EvalError;
pub use parser::ParseError;

use self::value::EmptyContext;

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

    pub fn parse(source: &str) -> Result<Expr, ParseError> {
        parser::parse(source)
    }

    pub fn eval<V: Value>(&self) -> Result<V, EvalError> {
        evaluate::evaluate::<V, EmptyContext>(self, &EmptyContext::new())
    }

    pub fn eval_in<V: Value, C: Context<V>>(&self, c: &C) -> Result<V, EvalError> {
        evaluate::evaluate::<V, C>(self, c)
    }
}

