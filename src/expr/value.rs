
use std::ops::*;
use num::*;

#[derive(Clone)]
pub enum Value {
    Rational(BigRational),
    Float(f64),
}

impl Value {
    pub fn to_f64(&self) -> f64 {
        match self {
            Value::Rational(x) => x.to_f64().unwrap_or(f64::NAN),
            Value::Float(x) => *x,
        }
    }
}

fn apply_op<F, G>(lhs: &Value, rhs: &Value, rat: F, float: G) -> Value
where F: Fn(&BigRational, &BigRational) -> BigRational, G: Fn(f64, f64) -> f64
{
    if let (Value::Rational(a), Value::Rational(b)) = (lhs, rhs) {
        Value::Rational(rat(a, b))
    } else {
        Value::Float(float(lhs.to_f64(), rhs.to_f64()))
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Value {
        apply_op(&self, &rhs, |a, b| a.add(b), |a, b| a.add(b))
    }
}

impl Sub for Value {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        apply_op(&self, &rhs, |a, b| a.sub(b), |a, b| a.sub(b))
    }
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        apply_op(&self, &rhs, |a, b| a.mul(b), |a, b| a.mul(b))
    }
}

impl Div for Value {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        if rhs.is_zero() {
            Value::Float(f64::INFINITY)
        } else {
            apply_op(&self, &rhs, |a, b| a.div(b), |a, b| a.div(b))
        }
    }
}

impl Rem for Value {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        if rhs.is_zero() {
            Value::Float(f64::NAN)
        } else {
            apply_op(&self, &rhs, |a, b| a.rem(b), |a, b| a.rem(b))
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, rhs: &Value) -> bool {
        if let (Value::Rational(a), Value::Rational(b)) = (self, rhs) {
            a.eq(&b)
        } else {
            self.to_f64().eq(&rhs.to_f64())
        }
    }
}

impl Zero for Value {
    fn zero() -> Self {
        Value::Rational(BigRational::zero())
    }

    fn is_zero(&self) -> bool {
        match self {
            Value::Rational(x) => x.is_zero(),
            Value::Float(x) => x == &0.0,
        }
    }
}

impl One for Value {
    fn one() -> Self {
        Value::Rational(BigRational::one())
    }

    fn is_one(&self) -> bool {
        match self {
            Value::Rational(x) => x.is_one(),
            Value::Float(x) => x == &1.0,
        }
    }
}

impl Num for Value {
    type FromStrRadixErr = <BigRational as Num>::FromStrRadixErr;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        Result::Ok(Value::Rational(BigRational::from_str_radix(str, radix)?))
    }
}

