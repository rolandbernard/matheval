
use std::collections::HashMap;

use crate::Context;
use crate::ContextFn;

use super::Number;

pub struct  NumberContext {
    vars: HashMap<String, Number>,
    funcs: HashMap<String, ContextFn<Number>>,
}

impl Context<Number> for NumberContext{
    fn set_variable(&mut self, name: &str, value: Number) {
        self.vars.insert(name.to_owned(), value);
    }

    fn set_function(&mut self, name: &str, value: ContextFn<Number>) {
        self.funcs.insert(name.to_owned(), value);
    }

    fn get_variable(&self, name: &str) -> Option<Number> {
        if let Some(v) = self.vars.get(name) {
            return Some(v.clone());
        } else {
            return match name {
                "pi" => Some(Number::pi()),
                "e" => Some(Number::e()),
                _ => None,
            }
        }
    }

    fn get_function<'a>(&'a self, name: &str) -> Option<&'a ContextFn<Number>> {
        if let Some(f) = self.funcs.get(name) {
            return Some(f);
        } else {
            return match name {
                _ => None,
            }
        }
    }
}

