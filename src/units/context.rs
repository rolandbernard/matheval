
use std::{collections::HashMap, cmp::Ordering};

use crate::{Context, ContextFn, EvalError, Number};

use super::Quantity;

pub struct QuantityContext {
    vars: HashMap<String, Quantity>,
    funcs: HashMap<String, ContextFn<Quantity>>,
}

fn check_length(args: Vec<Quantity>, min: usize, max: usize) -> Result<Vec<Quantity>, EvalError> {
    if args.len() < min {
        return Err(EvalError::ArgumentMismatch("Too few arguments to function".to_owned()));
    } else if args.len() > max {
        return Err(EvalError::ArgumentMismatch("Too many arguments to function".to_owned()));
    } else {
        return Ok(args);
    }
}

fn min(mut args: Vec<Quantity>) -> Result<Quantity, EvalError> {
    args = check_length(args, 1, usize::MAX)?;
    let mut m = 0;
    for i in 1..args.len() {
        let ord = args[i].partial_cmp(&args[m]);
        if let Some(o) = ord {
            if o == Ordering::Less {
                m = i;
            }
        } else {
            return Err(EvalError::NotSupported("Values in min function are not comparable".to_owned()));
        }
    }
    return Ok(args[m].clone());
}

fn max(mut args: Vec<Quantity>) -> Result<Quantity, EvalError> {
    args = check_length(args, 1, usize::MAX)?;
    let mut m = 0;
    for i in 1..args.len() {
        let ord = args[i].partial_cmp(&args[m]);
        if let Some(o) = ord {
            if o == Ordering::Greater {
                m = i;
            }
        } else {
            return Err(EvalError::NotSupported("Values in max function are not comparable".to_owned()));
        }
    }
    return Ok(args[m].clone());
}

fn unitless_function<F: Fn(&Number) -> Number>(mut vec: Vec<Quantity>, f: F) -> Result<Quantity, EvalError> {
    vec = check_length(vec, 1, 1)?;
    if vec[0].is_unitless() {
        let num = f(vec[0].coefficient());
        return num.nan_to_err().map(|x| Quantity::unitless(x));
    } else {
        return Err(EvalError::NotSupported("Function can only be applied to unitless quantity".to_owned()));
    }
}

impl QuantityContext {
    pub fn new() -> QuantityContext {
        let mut res = QuantityContext { vars: HashMap::new(), funcs: HashMap::new() };
        res.set_variable("pi", Quantity::pi());
        res.set_variable("e", Quantity::e());
        res.set_function("abs", Box::new(|v| check_length(v, 1, 1)?[0].abs().nan_to_err()));
        res.set_function("sign", Box::new(|v| check_length(v, 1, 1)?[0].sign().nan_to_err()));
        res.set_function("sqrt", Box::new(|v| check_length(v, 1, 1)?[0].sqrt().nan_to_err()));
        res.set_function("cbrt", Box::new(|v| check_length(v, 1, 1)?[0].cbrt().nan_to_err()));
        res.set_function("min", Box::new(min));
        res.set_function("max", Box::new(max));
        res.set_function("floor",Box::new(|v| unitless_function(v, Number::floor)));
        res.set_function("ceil",Box::new(|v| unitless_function(v, Number::ceil)));
        res.set_function("round", Box::new(|v| unitless_function(v, Number::round)));
        res.set_function("trunc", Box::new(|v| unitless_function(v, Number::trunc)));
        res.set_function("fract", Box::new(|v| unitless_function(v, Number::fract)));
        res.set_function("ln", Box::new(|v| unitless_function(v, Number::ln)));
        res.set_function("log", Box::new(|v| unitless_function(v, Number::log)));
        res.set_function("sin", Box::new(|v| unitless_function(v, Number::sin)));
        res.set_function("cos", Box::new(|v| unitless_function(v, Number::cos)));
        res.set_function("tan", Box::new(|v| unitless_function(v, Number::tan)));
        res.set_function("asin", Box::new(|v| unitless_function(v, Number::asin)));
        res.set_function("acos", Box::new(|v| unitless_function(v, Number::acos)));
        res.set_function("atan", Box::new(|v| unitless_function(v, Number::atan)));
        res.set_function("atan2", Box::new(|mut v| {
            v = check_length(v, 2, 2)?;
            if v[0].is_unitless() && v[1].is_unitless() {
                let num = v[0].coefficient().atan2(v[1].coefficient());
                return num.nan_to_err().map(|x| Quantity::unitless(x));
            } else {
                return Err(EvalError::NotSupported("Function can only be applied to unitless quantity".to_owned()));
            }
        }));
        res.set_function("sinh", Box::new(|v| unitless_function(v, Number::sinh)));
        res.set_function("cosh", Box::new(|v| unitless_function(v, Number::cosh)));
        res.set_function("tanh", Box::new(|v| unitless_function(v, Number::tanh)));
        res.set_function("asinh", Box::new(|v| unitless_function(v, Number::asinh)));
        res.set_function("acosh", Box::new(|v| unitless_function(v, Number::acosh)));
        res.set_function("atanh", Box::new(|v| unitless_function(v, Number::atanh)));
        return res;
    }
}

impl Context<Quantity> for QuantityContext {
    fn set_variable(&mut self, name: &str, value: Quantity) {
        self.vars.insert(name.to_owned(), value);
    }

    fn set_function(&mut self, name: &str, value: ContextFn<Quantity>) {
        self.funcs.insert(name.to_owned(), value);
    }

    fn get_variable(&self, name: &str) -> Option<Quantity> {
        return self.vars.get(name).and_then(|n| Some(n.clone()));
    }

    fn get_function<'a>(&'a self, name: &str) -> Option<&'a ContextFn<Quantity>> {
        return self.funcs.get(name);
    }
}

