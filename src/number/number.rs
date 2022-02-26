
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

    pub fn to_rational(&self) -> BigRational {
        match self {
            Number::Rational(r) => r.clone(),
            Number::Float(f) => BigRational::from_f64(*f).unwrap(),
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

impl Value for Number {
    fn parse_from(s: &str) -> Result<Self, EvalError> {
        #[derive(PartialEq)]
        enum Phase {
            Int, Frac, Exp
        }
        let mut phase = Phase::Int;
        let mut num = BigInt::zero();
        let mut den = BigInt::one();
        let mut exp = BigUint::zero();
        let mut next_exp = false;
        let mut base = 10;
        let mut start = 0;
        for (i, c) in s.chars().enumerate() {
            match c {
                'b' | 'o' | 'x' => {
                    if !num.is_zero() || phase != Phase::Int || base != 10 {
                        return Err(EvalError::InvalidLiteral("Radix indication character after non-zero characters".to_owned()));
                    } else if c == 'b' {
                        base = 2;
                    } else if c == 'o' {
                        base = 8;
                    } else {
                        base = 16;
                    }
                },
                '-' | '+' => {
                    if i != start {
                        return Err(EvalError::InvalidLiteral("'-' or '+' characters not at beginning".to_owned()));
                    } else if c == '-' {
                        if phase == Phase::Int {
                            den = -den;
                        } else {
                            next_exp = true;
                        }
                    }
                },
                '.' => {
                    if phase != Phase::Int {
                        return Err(EvalError::InvalidLiteral("Unexpected '.' characters".to_owned()));
                    } else {
                        phase = Phase::Frac;
                    }
                },
                'e' => {
                    if phase == Phase::Exp {
                        return Err(EvalError::InvalidLiteral("Unexpected 'e' characters".to_owned()));
                    } else {
                        phase = Phase::Exp;
                        start = i + 1;
                    }
                }
                _ => {
                    if phase == Phase::Exp {
                        exp *= base;
                        if let Some(d) = c.to_digit(base) {
                            exp += d;
                        } else {
                            return Err(EvalError::InvalidLiteral("Literal contains non-digit characters".to_owned()));
                        }
                    } else {
                        if phase == Phase::Frac {
                            den *= base;
                        }
                        num *= base;
                        if let Some(d) = c.to_digit(base) {
                            num += d;
                        } else {
                            return Err(EvalError::InvalidLiteral("Literal contains non-digit characters".to_owned()));
                        }
                    }
                },
            }
        }
        if next_exp {
            den *= BigInt::from_u32(base).unwrap().pow(exp);
        } else {
            num *= BigInt::from_u32(base).unwrap().pow(exp);
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

