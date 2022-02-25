
use std::collections::HashMap;

use crate::Context;
use crate::ContextFn;
use crate::EvalError;

use super::Number;

pub struct NumberContext {
    vars: HashMap<String, Number>,
    funcs: HashMap<String, ContextFn<Number>>,
}

fn check_length(args: &[Number], min: usize, max: usize) -> Result<&[Number], EvalError> {
    if args.len() < min {
        return Err(EvalError::ArgumentMismatch("Too few arguments to function".to_owned()));
    } else if args.len() > max {
        return Err(EvalError::ArgumentMismatch("Too many arguments to function".to_owned()));
    } else {
        return Ok(args);
    }
}

fn min(args: &[Number]) -> Result<Number, EvalError> {
    check_length(args, 1, usize::MAX)?;
    let mut m = 0;
    for i in 1..args.len() {
        if args[i] < args[m] {
            m = i;
        }
    }
    return Ok(args[m].clone());
}

fn max(args: &[Number]) -> Result<Number, EvalError> {
    check_length(args, 1, usize::MAX)?;
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
        res.vars.insert("pi".to_owned(), Number::pi());
        res.vars.insert("e".to_owned(), Number::e());
        res.funcs.insert("floor".to_owned(),Box::new(|v| Ok(check_length(v, 1, 1)?[0].floor())));
        res.funcs.insert("ceil".to_owned(),Box::new(|v| Ok(check_length(v, 1, 1)?[0].ceil())));
        res.funcs.insert("round".to_owned(), Box::new(|v| Ok(check_length(v, 1, 1)?[0].round())));
        res.funcs.insert("trunc".to_owned(), Box::new(|v| Ok(check_length(v, 1, 1)?[0].trunc())));
        res.funcs.insert("fract".to_owned(), Box::new(|v| Ok(check_length(v, 1, 1)?[0].fract())));
        res.funcs.insert("abs".to_owned(), Box::new(|v| Ok(check_length(v, 1, 1)?[0].abs())));
        res.funcs.insert("sign".to_owned(), Box::new(|v| Ok(check_length(v, 1, 1)?[0].sign())));
        res.funcs.insert("sqrt".to_owned(), Box::new(|v| Ok(check_length(v, 1, 1)?[0].sqrt())));
        res.funcs.insert("ln".to_owned(), Box::new(|v| Ok(check_length(v, 1, 1)?[0].ln())));
        res.funcs.insert("log".to_owned(), Box::new(|v| Ok(check_length(v, 1, 1)?[0].log())));
        res.funcs.insert("cbrt".to_owned(), Box::new(|v| Ok(check_length(v, 1, 1)?[0].cbrt())));
        res.funcs.insert("sin".to_owned(), Box::new(|v| Ok(check_length(v, 1, 1)?[0].sin())));
        res.funcs.insert("cos".to_owned(), Box::new(|v| Ok(check_length(v, 1, 1)?[0].cos())));
        res.funcs.insert("tan".to_owned(), Box::new(|v| Ok(check_length(v, 1, 1)?[0].tan())));
        res.funcs.insert("asin".to_owned(), Box::new(|v| Ok(check_length(v, 1, 1)?[0].asin())));
        res.funcs.insert("acos".to_owned(), Box::new(|v| Ok(check_length(v, 1, 1)?[0].acos())));
        res.funcs.insert("atan".to_owned(), Box::new(|v| Ok(check_length(v, 1, 1)?[0].atan())));
        res.funcs.insert("atan2".to_owned(), Box::new(|v| Ok(check_length(v, 2, 2)?[0].atan2(&v[1]))));
        res.funcs.insert("sinh".to_owned(), Box::new(|v| Ok(check_length(v, 1, 1)?[0].sinh())));
        res.funcs.insert("cosh".to_owned(), Box::new(|v| Ok(check_length(v, 1, 1)?[0].cosh())));
        res.funcs.insert("tanh".to_owned(), Box::new(|v| Ok(check_length(v, 1, 1)?[0].tanh())));
        res.funcs.insert("asinh".to_owned(), Box::new(|v| Ok(check_length(v, 1, 1)?[0].asinh())));
        res.funcs.insert("acosh".to_owned(), Box::new(|v| Ok(check_length(v, 1, 1)?[0].acosh())));
        res.funcs.insert("atanh".to_owned(), Box::new(|v| Ok(check_length(v, 1, 1)?[0].atanh())));
        res.funcs.insert("min".to_owned(), Box::new(min));
        res.funcs.insert("max".to_owned(), Box::new(max));
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

