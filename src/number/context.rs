
use std::cmp::Ordering;
use std::collections::HashMap;

use crate::Context;
use crate::ContextFn;
use crate::EvalError;

use super::Number;

pub struct NumberContext {
    vars: HashMap<String, Number>,
    funcs: HashMap<String, ContextFn<Number>>,
}

fn check_length(args: Vec<Number>, min: usize, max: usize) -> Result<Vec<Number>, EvalError> {
    if args.len() < min {
        return Err(EvalError::ArgumentMismatch("Too few arguments to function".to_owned()));
    } else if args.len() > max {
        return Err(EvalError::ArgumentMismatch("Too many arguments to function".to_owned()));
    } else {
        return Ok(args);
    }
}

fn min(mut args: Vec<Number>) -> Result<Number, EvalError> {
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

fn max(mut args: Vec<Number>) -> Result<Number, EvalError> {
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

impl NumberContext {
    pub fn new() -> NumberContext {
        let mut res = NumberContext { vars: HashMap::new(), funcs: HashMap::new() };
        res.set_variable("pi", Number::pi());
        res.set_variable("e", Number::e());
        res.set_function("floor",Box::new(|v| check_length(v, 1, 1)?[0].floor().nan_to_err()));
        res.set_function("ceil",Box::new(|v| check_length(v, 1, 1)?[0].ceil().nan_to_err()));
        res.set_function("round", Box::new(|v| check_length(v, 1, 1)?[0].round().nan_to_err()));
        res.set_function("trunc", Box::new(|v| check_length(v, 1, 1)?[0].trunc().nan_to_err()));
        res.set_function("fract", Box::new(|v| check_length(v, 1, 1)?[0].fract().nan_to_err()));
        res.set_function("abs", Box::new(|v| check_length(v, 1, 1)?[0].abs().nan_to_err()));
        res.set_function("sign", Box::new(|v| check_length(v, 1, 1)?[0].sign().nan_to_err()));
        res.set_function("sqrt", Box::new(|v| check_length(v, 1, 1)?[0].sqrt().nan_to_err()));
        res.set_function("ln", Box::new(|v| check_length(v, 1, 1)?[0].ln().nan_to_err()));
        res.set_function("log", Box::new(|v| check_length(v, 1, 1)?[0].log().nan_to_err()));
        res.set_function("cbrt", Box::new(|v| check_length(v, 1, 1)?[0].cbrt().nan_to_err()));
        res.set_function("sin", Box::new(|v| check_length(v, 1, 1)?[0].sin().nan_to_err()));
        res.set_function("cos", Box::new(|v| check_length(v, 1, 1)?[0].cos().nan_to_err()));
        res.set_function("tan", Box::new(|v| check_length(v, 1, 1)?[0].tan().nan_to_err()));
        res.set_function("asin", Box::new(|v| check_length(v, 1, 1)?[0].asin().nan_to_err()));
        res.set_function("acos", Box::new(|v| check_length(v, 1, 1)?[0].acos().nan_to_err()));
        res.set_function("atan", Box::new(|v| check_length(v, 1, 1)?[0].atan().nan_to_err()));
        res.set_function("atan2", Box::new(|mut v| {
            v = check_length(v, 2, 2)?; v[0].atan2(&v[1])
        }.nan_to_err()));
        res.set_function("sinh", Box::new(|v| check_length(v, 1, 1)?[0].sinh().nan_to_err()));
        res.set_function("cosh", Box::new(|v| check_length(v, 1, 1)?[0].cosh().nan_to_err()));
        res.set_function("tanh", Box::new(|v| check_length(v, 1, 1)?[0].tanh().nan_to_err()));
        res.set_function("asinh", Box::new(|v| check_length(v, 1, 1)?[0].asinh().nan_to_err()));
        res.set_function("acosh", Box::new(|v| check_length(v, 1, 1)?[0].acosh().nan_to_err()));
        res.set_function("atanh", Box::new(|v| check_length(v, 1, 1)?[0].atanh().nan_to_err()));
        res.set_function("min", Box::new(min));
        res.set_function("max", Box::new(max));
        return res;
    }
}

impl Context<Number> for NumberContext {
    fn set_variable(&mut self, name: &str, value: Number) {
        self.vars.insert(name.to_owned(), value);
    }

    fn set_function(&mut self, name: &str, value: ContextFn<Number>) {
        self.funcs.insert(name.to_owned(), value);
    }

    fn get_variable(&self, name: &str) -> Option<Number> {
        return self.vars.get(name).and_then(|n| Some(n.clone()));
    }

    fn get_function<'a>(&'a self, name: &str) -> Option<&'a ContextFn<Number>> {
        return self.funcs.get(name);
    }
}

