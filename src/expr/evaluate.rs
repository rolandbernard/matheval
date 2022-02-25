
use super::Expr;
use super::Value;

#[derive(Debug)]
pub enum EvalError {
    MathError(String),
    InvalidLiteral(String),
    NotSupported(String),
}

pub fn evaluate<V: Value>(expr: &Expr) -> Result<V, EvalError> {
    match expr {
        Expr::Constant(s) => V::read_from(&s),
        Expr::Sum(args) => {
            let mut sum = evaluate::<V>(&args[0])?;
            for i in 1..args.len() {
                sum = sum.add(&evaluate::<V>(&args[i])?)?;
            }
            return Ok(sum);
        },
        Expr::Product(args) => {
            let mut product = evaluate::<V>(&args[0])?;
            for i in 1..args.len() {
                product = product.mul(&evaluate::<V>(&args[i])?)?;
            }
            return Ok(product);
        },
        Expr::Power(args) => {
            let len = args.len();
            let mut pow = evaluate::<V>(&args[len - 1])?;
            for i in (0..args.len() - 1).rev() {
                pow = evaluate::<V>(&args[i])?.pow(&pow)?;
            }
            return Ok(pow);
        },
        Expr::Function(_name, _args) => panic!(),
        Expr::Variable(_name) => panic!(),
    }
}

