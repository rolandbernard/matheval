
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
        Expr::Constant(s) => V::parse_from(&s),
        Expr::Sum(args) => {
            let mut sum = evaluate::<V, C>(&args[0], cnxt)?;
            for i in 1..args.len() {
                sum = sum.add(&evaluate::<V, C>(&args[i], cnxt)?)?;
            }
            return Ok(sum);
        },
        Expr::Product(args) => {
            let mut product = evaluate::<V, C>(&args[0], cnxt)?;
            for i in 1..args.len() {
                product = product.mul(&evaluate::<V, C>(&args[i], cnxt)?)?;
            }
            return Ok(product);
        },
        Expr::Power(args) => {
            let len = args.len();
            let mut pow = evaluate::<V, C>(&args[len - 1], cnxt)?;
            for i in (0..args.len() - 1).rev() {
                pow = evaluate::<V, C>(&args[i], cnxt)?.pow(&pow)?;
            }
            return Ok(pow);
        },
        Expr::Function(name, args) => {
            let mut argn = Vec::new();
            for a in args {
                argn.push(evaluate::<V, C>(a, cnxt)?);
            }
            if let Some(f) = cnxt.get_function(name) {
                return f(&argn);
            } else {
                return Err(EvalError::UnknownFunction("Function not found".to_owned()));
            }
        },
        Expr::Variable(name) => {
            if let Some(v) = cnxt.get_variable(name) {
                return Ok(v);
            } else {
                return Err(EvalError::UnknownVariable("Variable not found".to_owned()));
            }
        },
    }
}

