
mod value;
mod evaluate;
mod parser;
mod format;
mod expr;

pub use expr::Expr;
pub use value::Value;
pub use value::Context;
pub use value::ContextFn;
pub use evaluate::EvalError;
pub use parser::ParseError;

impl Expr {
    pub fn parse(source: &str) -> Result<Expr, ParseError> {
        parser::parse(source)
    }

    pub fn eval<V: Value>(&self) -> Result<V, EvalError> {
        self.eval_in::<V, V::DefaultContext>(&V::default_context())
    }

    pub fn eval_in<V: Value, C: Context<V>>(&self, c: &C) -> Result<V, EvalError> {
        evaluate::evaluate::<V, C>(self, c)
    }
}

impl ToString for Expr {
    fn to_string(&self) -> String {
        format::format(self)
    }
}

