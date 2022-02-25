
use std::ops::*;
use num::*;
use num::traits::Pow;

use crate::Value;
use crate::EvalError;

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
}

impl ToString for Number {
    fn to_string(&self) -> String {
        match self {
            Number::Rational(r) => r.to_string(),
            Number::Float(f) => f.to_string(),
        }
    }
}

impl Value for Number {
    fn read_from(s: &str) -> Result<Self, EvalError> {
        let mut num = BigInt::zero();
        let mut den = BigInt::one();
        let mut dot = false;
        for (i, c) in s.chars().enumerate() {
            if c == '-' || c == '+' {
                if i != 0 {
                    return Err(EvalError::InvalidLiteral("'-' or '+' characters not at beginning".to_owned()));
                } else if c == '-' {
                    den = -den;
                }
            } else if c == '.' {
                if dot {
                    return Err(EvalError::InvalidLiteral("Multiple '.' characters".to_owned()));
                }
                dot = true;
            } else {
                if dot {
                    den *= 10 as i64;
                }
                num *= 10 as i64;
                if let Some(d) = c.to_digit(10) {
                    num += d;
                } else {
                    return Err(EvalError::InvalidLiteral("Literal contains non-digit characters".to_owned()));
                }
            }
        }
        return Ok(Number::Rational(BigRational::new(num, den)));
    }

    fn add(&self, o: &Self) -> Result<Self, EvalError> {
        if let (Number::Rational(a), Number::Rational(b)) = (self, o) {
            return Ok(Number::Rational(a.add(b)));
        } else {
            return Ok(Number::Float(self.to_f64() + o.to_f64()));
        }
    }

    fn mul(&self, o: &Self) -> Result<Self, EvalError> {
        if let (Number::Rational(a), Number::Rational(b)) = (self, o) {
            return Ok(Number::Rational(a.mul(b)));
        } else {
            return Ok(Number::Float(self.to_f64() * o.to_f64()));
        }
    }

    fn pow(&self, o: &Self) -> Result<Self, EvalError> {
        if let (Number::Rational(a), Number::Rational(b)) = (self, o) {
            if b.is_integer() {
                if let Some(i) = b.to_i32() {
                    return Ok(Number::Rational(a.pow(i)));
                }
            }
        } 
        return Ok(Number::Float(self.to_f64().pow(o.to_f64())));
    }
}

