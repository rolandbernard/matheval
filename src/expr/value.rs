
#[derive(Debug)]
pub enum EvalError {
    MathError(String),
    InvalidLiteral(String),
    NotSupported(String),
}

pub trait Value
where Self: Sized + ToString {
    fn read_from(s: &str) -> Result<Self, EvalError>;

    fn add(self, o: Self) -> Result<Self, EvalError>;

    fn mul(self, o: Self) -> Result<Self, EvalError>;

    fn pow(self, o: Self) -> Result<Self, EvalError>;
}

