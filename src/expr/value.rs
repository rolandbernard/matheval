
use super::EvalError;

pub trait Value
where Self: Sized + ToString {
    fn read_from(s: &str) -> Result<Self, EvalError>;

    fn add(&self, o: &Self) -> Result<Self, EvalError>;

    fn mul(&self, o: &Self) -> Result<Self, EvalError>;

    fn pow(&self, o: &Self) -> Result<Self, EvalError>;
}

pub struct ContextFn<V: Value> {
    pub argc: usize,
    pub func: Box<dyn Fn(&[V]) -> Result<V, EvalError>>,
}

pub trait Context<V: Value> {
    fn set_variable(&mut self, name: &str, value: V);

    fn set_function(&mut self, name: &str, value: ContextFn<V>);

    fn get_variable(&self, name: &str) -> Option<V>;

    fn get_function<'a>(&'a self, name: &str) -> Option<&'a ContextFn<V>>;
}

