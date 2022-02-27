
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
                'e' if base == 10 => {
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

