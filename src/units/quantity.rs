
use std::ops::*;
use std::str::FromStr;
use num::traits::Pow;

use crate::{Number, Value, QuantityContext, EvalError};

use super::Unit;

#[derive(PartialEq, Clone, Debug)]
pub struct Quantity {
    number: Number,
    unit: Unit,
}

impl Quantity {
    pub fn new(number: Number, unit: Unit) -> Quantity {
        Quantity { number, unit }
    }

    pub fn unitless(number: Number) -> Quantity {
        Quantity { number, unit: Unit::empty() }
    }

    pub fn pi() -> Quantity {
        Quantity::unitless(Number::pi())
    }

    pub fn e() -> Quantity {
        Quantity::unitless(Number::e())
    }
    
    pub fn abs(&self) -> Quantity {
        Quantity::new(self.number.abs(), self.unit.clone())
    }

    pub fn sign(&self) -> Quantity {
        Quantity::unitless(self.number.sign())
    }

    pub fn sqrt(&self) -> Quantity {
        Quantity::new(self.number.sqrt(), self.unit.clone().pow(Number::from_i64s(1, 2)))
    }

    pub fn cbrt(&self) -> Quantity {
        Quantity::new(self.number.cbrt(), self.unit.clone().pow(Number::from_i64s(1, 3)))
    }
}

impl PartialOrd for Quantity {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.unit == other.unit {
            self.number.partial_cmp(&other.number)
        } else {
            None
        }
    }
}

impl ToString for Quantity {
    fn to_string(&self) -> String {
        format!("{} {}", self.number.to_string(), self.unit.to_string())
    }
}

impl FromStr for Quantity {
    type Err = EvalError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Quantity::unitless(Number::from_str(s)?))
    }
}

impl Neg for Quantity {
    type Output = Result<Quantity, EvalError>;

    fn neg(mut self) -> Self::Output {
        self.number = self.number.neg()?;
        return Ok(self);
    }
}

impl Add for Quantity {
    type Output = Result<Quantity, EvalError>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.unit != rhs.unit {
            return Err(EvalError::UnitError(format!("Cannot add {} to {}", self.unit.to_string(), rhs.unit.to_string())));
        } else {
            return Ok(Quantity { number: self.number.add(rhs.number)?, unit: self.unit });
        }
    }
}

impl Sub for Quantity {
    type Output = Result<Quantity, EvalError>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.unit != rhs.unit {
            return Err(EvalError::UnitError(format!("Cannot subtract {} to {}", self.unit.to_string(), rhs.unit.to_string())));
        } else {
            return Ok(Quantity { number: self.number.sub(rhs.number)?, unit: self.unit });
        }
    }
}

impl Mul for Quantity {
    type Output = Result<Quantity, EvalError>;

    fn mul(self, rhs: Self) -> Self::Output {
        return Ok(Quantity { number: self.number.mul(rhs.number)?, unit: self.unit.mul(rhs.unit) });
    }
}

impl Div for Quantity {
    type Output = Result<Quantity, EvalError>;

    fn div(self, rhs: Self) -> Self::Output {
        return Ok(Quantity { number: self.number.div(rhs.number)?, unit: self.unit.div(rhs.unit) });
    }
}

impl Pow<Quantity> for Quantity {
    type Output = Result<Quantity, EvalError>;

    fn pow(self, rhs: Quantity) -> Self::Output {
        if !rhs.unit.is_empty() {
            return Err(EvalError::UnitError(format!("Cannot take power with exponent of {}", rhs.unit.to_string())));
        } else {
            return Ok(Quantity { number: self.number.pow(rhs.number.clone())?, unit: self.unit.pow(rhs.number) });
        }
    }
}

impl Value for Quantity {
    type DefaultContext = QuantityContext;

    fn default_context() -> Self::DefaultContext {
        QuantityContext::new()
    }
}

