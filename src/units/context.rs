
use std::collections::HashMap;

use crate::Context;
use crate::ContextFn;

use super::Quantity;

pub struct QuantityContext {
    vars: HashMap<String, Quantity>,
    funcs: HashMap<String, ContextFn<Quantity>>,
}

impl QuantityContext {
    pub fn new() -> QuantityContext {
        let mut res = QuantityContext { vars: HashMap::new(), funcs: HashMap::new() };
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

