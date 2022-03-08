
use std::str::FromStr;
use std::ops::*;
use num::traits::Pow;

use super::EvalError;

pub trait Value
where Self:
    Sized + ToString
    + FromStr<Err = EvalError> + Add<Output = Result<Self, EvalError>>
    + Sub<Output = Result<Self, EvalError>> + Mul<Output = Result<Self, EvalError>>
    + Div<Output = Result<Self, EvalError>> + Neg<Output = Result<Self, EvalError>>
    + Pow<Self, Output = Result<Self, EvalError>> + PartialOrd
{
    type DefaultContext: Context<Self>;

    fn default_context() -> Self::DefaultContext;
}

pub type ContextFn<V> = dyn Fn(Vec<V>) -> Result<V, EvalError>;

pub trait Context<V: Value> {
    fn set_variable(&mut self, name: &str, value: V);

    fn set_function(&mut self, name: &str, value: Box<ContextFn<V>>);

    fn get_variable(&self, name: &str) -> Option<V>;

    fn get_function<'a>(&'a self, name: &str) -> Option<&'a ContextFn<V>>;
}

