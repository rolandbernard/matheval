
use std::ops::*;
use std::str::FromStr;
use num::*;
use num::traits::Pow;

use crate::Value;
use crate::EvalError;

use super::NumberContext;

#[derive(Debug, Clone)]
pub enum Number {
    Rational(BigRational),
    Float(f64),
}

impl Number {
    pub fn to_f64(&self) -> f64 {
        match self {
            Number::Rational(r) => r.to_f64().unwrap_or(f64::NAN),
            Number::Float(f) => *f,
        }
    }

    pub fn to_rational(&self) -> BigRational {
        match self {
            Number::Rational(r) => r.clone(),
            Number::Float(f) => BigRational::from_f64(*f).unwrap(),
        }
    }

    pub fn is_rational(&self) -> bool {
        match self {
            Number::Rational(..) => true,
            Number::Float(..) => false,
        }
    }

    pub fn is_integer(&self) -> bool {
        match self {
            Number::Rational(r) => r.is_integer(),
            Number::Float(f) => *f == f.trunc(),
        }
    }

    pub fn is_positive(&self) -> bool {
        match self {
            Number::Rational(r) => r.is_positive(),
            Number::Float(f) => *f > 0.0,
        }
    }

    pub fn is_negative(&self) -> bool {
        match self {
            Number::Rational(r) => r.is_negative(),
            Number::Float(f) => *f < 0.0,
        }
    }

    pub fn is_zero(&self) -> bool {
        match self {
            Number::Rational(r) => r.is_zero(),
            Number::Float(f) => f.is_zero(),
        }
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Float(l), Self::Float(r)) => l == r,
            (Self::Rational(l), Self::Rational(r)) => l == r,
            (l, r) => l.to_rational() == r.to_rational(),
        }
    }
}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Float(l), Self::Float(r)) => l.partial_cmp(r),
            (Self::Rational(l), Self::Rational(r)) => l.partial_cmp(r),
            (l, r) => l.to_rational().partial_cmp(&r.to_rational()),
        }
    }
}

impl ToString for Number {
    fn to_string(&self) -> String {
        match self {
            Number::Rational(r) => r.to_string(),
            Number::Float(f) => f.to_string(),
        }
    }
}

impl FromStr for Number {
    type Err = EvalError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s.chars().collect::<Vec<_>>();
        let mut pos = 0;
        let mut num = BigInt::zero();
        let mut den = BigInt::one();
        if pos < chars.len() && (chars[pos] == '-' || chars[pos] == '+') {
            if chars[pos] == '-' {
                den = -den;
            }
            pos += 1;
        }
        let base;
        if pos + 2 < chars.len() && chars[pos] == '0' && chars[pos + 1] == 'b' && chars[pos + 2].is_digit(2) {
            pos += 2;
            base = 2;
        } else if pos + 2 < chars.len() && chars[pos] == '0' && chars[pos + 1] == 'o' && chars[pos + 2].is_digit(8) {
            pos += 2;
            base = 8;
        } else if pos + 2 < chars.len() && chars[pos] == '0' && chars[pos + 1] == 'x' && chars[pos + 2].is_digit(16) {
            pos += 2;
            base = 16;
        } else if pos < chars.len() && chars[pos].is_digit(10) {
            base = 10;
        } else {
            return Err(EvalError::InvalidLiteral("Literals must not be empty".to_owned()));
        }
        while pos < chars.len() && chars[pos].is_digit(base) {
            num = base * num + chars[pos].to_digit(base).unwrap();
            pos += 1;
        }
        if pos + 1 < chars.len() && chars[pos] == '.' && chars[pos + 1].is_digit(base) {
            pos += 1;
            while pos < chars.len() && chars[pos].is_digit(base) {
                num = base * num + chars[pos].to_digit(base).unwrap();
                den *= base;
                pos += 1;
            }
        }
        if base == 10 && pos < chars.len() && chars[pos] == 'e' {
            let neg;
            if pos + 1 < chars.len() && chars[pos + 1].is_digit(base) {
                neg = false;
                pos += 1;
            } else if pos + 2 < chars.len() && (chars[pos + 1] == '+' || chars[pos + 1] == '-') && chars[pos + 2].is_digit(base) {
                if chars[pos + 1] == '-' {
                    neg = true;
                } else {
                    neg = false;
                }
                pos += 2;
            } else {
                return Err(EvalError::InvalidLiteral("Missing exponent".to_owned()));
            }
            let mut exp = BigUint::zero();
            while pos < chars.len() && chars[pos].is_digit(base) {
                exp = base * exp + chars[pos].to_digit(base).unwrap();
                pos += 1;
            }
            if neg {
                den *= BigInt::from_u32(base).unwrap().pow(exp);
            } else {
                num *= BigInt::from_u32(base).unwrap().pow(exp);
            }
        }
        if pos != chars.len() {
            return Err(EvalError::InvalidLiteral(format!("Unexpected character '{}'", chars[pos])));
        } else {
            return Ok(Number::Rational(BigRational::new(num, den)));
        }
    }
}

impl Neg for Number {
    type Output = Result<Number, EvalError>;

    fn neg(self) -> Self::Output {
        match self {
            Number::Rational(r) => Ok(Number::Rational(r.neg())),
            Number::Float(f) => Ok(Number::Float(f.neg())),
        }
    }
}

impl Add for Number {
    type Output = Result<Number, EvalError>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::Rational(a), Number::Rational(b)) => Ok(Number::Rational(a.add(b))),
            (a, b) => Ok(Number::Float(a.to_f64().add(b.to_f64()))),
        }
    }
}

impl Sub for Number {
    type Output = Result<Number, EvalError>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::Rational(a), Number::Rational(b)) => Ok(Number::Rational(a.sub(b))),
            (a, b) => Ok(Number::Float(a.to_f64().sub(b.to_f64()))),
        }
    }
}

impl Mul for Number {
    type Output = Result<Number, EvalError>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::Rational(a), Number::Rational(b)) => Ok(Number::Rational(a.mul(b))),
            (a, b) => Ok(Number::Float(a.to_f64().mul(b.to_f64()))),
        }
    }
}

impl Div for Number {
    type Output = Result<Number, EvalError>;

    fn div(self, rhs: Self) -> Self::Output {
        if rhs.is_zero() {
            Err(EvalError::MathError("Division by zero".to_owned()))
        } else {
            match (self, rhs) {
                (Number::Rational(a), Number::Rational(b)) => Ok(Number::Rational(a.div(b))),
                (a, b) => Ok(Number::Float(a.to_f64().div(b.to_f64()))),
            }
        }
    }
}

impl Pow<Number> for Number {
    type Output = Result<Number, EvalError>;

    fn pow(self, rhs: Number) -> Self::Output {
        if self.is_zero() && rhs.is_negative() {
            Err(EvalError::MathError("Division by zero".to_owned()))
        } else if self.is_zero() && rhs.is_zero() {
            Err(EvalError::MathError("Zero to the power of zero".to_owned()))
        } else {
            match (self, rhs) {
                (Number::Rational(a), Number::Rational(b)) if b.is_integer() => {
                    if let Some(i) = b.to_i32() {
                        Ok(Number::Rational(a.pow(i)))
                    } else {
                        Ok(Number::Float(a.to_f64().unwrap().pow(b.to_f64().unwrap())))
                    }
                },
                (a, b) => Ok(Number::Float(a.to_f64().pow(b.to_f64()))),
            }
        }
    }
}

impl Value for Number {
    type DefaultContext = NumberContext;

    fn default_context() -> Self::DefaultContext {
        NumberContext::new()
    }
}

