
use super::Expr;
use super::Value;
use super::Context;

#[derive(Debug)]
pub enum EvalError {
    MathError(String),
    InvalidLiteral(String),
    NotSupported(String),
    ArgumentMismatch(String),
    UnknownVariable(String),
    UnknownFunction(String),
}

pub fn evaluate<V: Value, C: Context<V>>(expr: &Expr, cnxt: &C) -> Result<V, EvalError> {
    match expr {
        Expr::Literal(s) => s.parse(),
        Expr::Add(l, r) => {
            evaluate::<V, C>(l, cnxt)?.add(evaluate::<V, C>(r, cnxt)?)
        },
        Expr::Sub(l, r) => {
            evaluate::<V, C>(l, cnxt)?.sub(evaluate::<V, C>(r, cnxt)?)
        },
        Expr::Mul(l, r) => {
            evaluate::<V, C>(l, cnxt)?.mul(evaluate::<V, C>(r, cnxt)?)
        },
        Expr::Div(l, r) => {
            evaluate::<V, C>(l, cnxt)?.div(evaluate::<V, C>(r, cnxt)?)
        },
        Expr::Neg(o) => {
            evaluate::<V, C>(o, cnxt)?.neg()
        },
        Expr::Pow(l, r) => {
            evaluate::<V, C>(l, cnxt)?.pow(evaluate::<V, C>(r, cnxt)?)
        },
        Expr::Function(name, args) => {
            let mut argn = Vec::new();
            for a in args {
                argn.push(evaluate::<V, C>(a, cnxt)?);
            }
            if let Some(f) = cnxt.get_function(name) {
                f(argn)
            } else {
                Err(EvalError::UnknownFunction("Function not found".to_owned()))
            }
        },
        Expr::Variable(name) => {
            if let Some(v) = cnxt.get_variable(name) {
                Ok(v)
            } else {
                Err(EvalError::UnknownVariable("Variable not found".to_owned()))
            }
        },
    }
}

