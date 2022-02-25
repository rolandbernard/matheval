
use super::EvalError;

pub trait Value
where Self: Sized + ToString {
    fn parse_from(s: &str) -> Result<Self, EvalError>;

    fn add(&self, o: &Self) -> Result<Self, EvalError>;

    fn mul(&self, o: &Self) -> Result<Self, EvalError>;

    fn pow(&self, o: &Self) -> Result<Self, EvalError>;
}

pub type ContextFn<V> = Box<dyn Fn(&[V]) -> Result<V, EvalError>>;

pub trait Context<V: Value> {
    fn set_variable(&mut self, name: &str, value: V);

    fn set_function(&mut self, name: &str, value: ContextFn<V>);

    fn get_variable(&self, name: &str) -> Option<V>;

    fn get_function<'a>(&'a self, name: &str) -> Option<&'a ContextFn<V>>;
}

pub struct EmptyContext { }

impl EmptyContext {
    pub fn new() -> EmptyContext {
        return EmptyContext {}
    }
}

impl<V: Value> Context<V> for EmptyContext {
    fn set_variable(&mut self, _name: &str, _value: V) { }

    fn set_function(&mut self, _name: &str, _value: ContextFn<V>) { }

    fn get_variable(&self, _name: &str) -> Option<V> { None }

    fn get_function<'a>(&'a self, _name: &str) -> Option<&'a ContextFn<V>> { None }
}

