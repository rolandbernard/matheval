
#[derive(PartialEq, Debug)]
pub enum Value {
    Rational(i64, u64),
    Float(f64),
}

