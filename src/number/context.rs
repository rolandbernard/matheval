
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
        if args[i] < args[m] {
            m = i;
        }
    }
    return Ok(args[m].clone());
}

fn max(mut args: Vec<Number>) -> Result<Number, EvalError> {
    args = check_length(args, 1, usize::MAX)?;
    let mut m = 0;
    for i in 1..args.len() {
        if args[i] > args[m] {
            m = i;
        }
    }
    return Ok(args[m].clone());
}

impl NumberContext {
    pub fn new() -> NumberContext {
        let mut res = NumberContext { vars: HashMap::new(), funcs: HashMap::new() };
        res.set_variable("pi", Number::pi());
        res.set_variable("e", Number::e());
        res.set_function("floor",Box::new(|v| Ok(check_length(v, 1, 1)?[0].floor())));
        res.set_function("ceil",Box::new(|v| Ok(check_length(v, 1, 1)?[0].ceil())));
        res.set_function("round", Box::new(|v| Ok(check_length(v, 1, 1)?[0].round())));
        res.set_function("trunc", Box::new(|v| Ok(check_length(v, 1, 1)?[0].trunc())));
        res.set_function("fract", Box::new(|v| Ok(check_length(v, 1, 1)?[0].fract())));
        res.set_function("abs", Box::new(|v| Ok(check_length(v, 1, 1)?[0].abs())));
        res.set_function("sign", Box::new(|v| Ok(check_length(v, 1, 1)?[0].sign())));
        res.set_function("sqrt", Box::new(|v| Ok(check_length(v, 1, 1)?[0].sqrt())));
        res.set_function("ln", Box::new(|v| Ok(check_length(v, 1, 1)?[0].ln())));
        res.set_function("log", Box::new(|v| Ok(check_length(v, 1, 1)?[0].log())));
        res.set_function("cbrt", Box::new(|v| Ok(check_length(v, 1, 1)?[0].cbrt())));
        res.set_function("sin", Box::new(|v| Ok(check_length(v, 1, 1)?[0].sin())));
        res.set_function("cos", Box::new(|v| Ok(check_length(v, 1, 1)?[0].cos())));
        res.set_function("tan", Box::new(|v| Ok(check_length(v, 1, 1)?[0].tan())));
        res.set_function("asin", Box::new(|v| Ok(check_length(v, 1, 1)?[0].asin())));
        res.set_function("acos", Box::new(|v| Ok(check_length(v, 1, 1)?[0].acos())));
        res.set_function("atan", Box::new(|v| Ok(check_length(v, 1, 1)?[0].atan())));
        res.set_function("atan2", Box::new(|mut v| Ok({
            v = check_length(v, 2, 2)?; v[0].atan2(&v[1])
        })));
        res.set_function("sinh", Box::new(|v| Ok(check_length(v, 1, 1)?[0].sinh())));
        res.set_function("cosh", Box::new(|v| Ok(check_length(v, 1, 1)?[0].cosh())));
        res.set_function("tanh", Box::new(|v| Ok(check_length(v, 1, 1)?[0].tanh())));
        res.set_function("asinh", Box::new(|v| Ok(check_length(v, 1, 1)?[0].asinh())));
        res.set_function("acosh", Box::new(|v| Ok(check_length(v, 1, 1)?[0].acosh())));
        res.set_function("atanh", Box::new(|v| Ok(check_length(v, 1, 1)?[0].atanh())));
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

