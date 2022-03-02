
use num::Signed;

use super::Number;

impl Number {
    pub fn floor(&self) -> Number {
        match self {
            Number::Rational(r) => Number::Rational(r.floor()),
            Number::Float(f) => Number::Float(f.floor()),
        }
    }

    pub fn ceil(&self) -> Number {
        match self {
            Number::Rational(r) => Number::Rational(r.ceil()),
            Number::Float(f) => Number::Float(f.ceil()),
        }
    }

    pub fn round(&self) -> Number {
        match self {
            Number::Rational(r) => Number::Rational(r.round()),
            Number::Float(f) => Number::Float(f.round()),
        }
    }

    pub fn trunc(&self) -> Number {
        match self {
            Number::Rational(r) => Number::Rational(r.trunc()),
            Number::Float(f) => Number::Float(f.trunc()),
        }
    }

    pub fn fract(&self) -> Number {
        match self {
            Number::Rational(r) => Number::Rational(r.fract()),
            Number::Float(f) => Number::Float(f.fract()),
        }
    }

    pub fn abs(&self) -> Number {
        match self {
            Number::Rational(r) => Number::Rational(r.abs()),
            Number::Float(f) => Number::Float(f.abs()),
        }
    }

    pub fn sign(&self) -> Number {
        if self.is_positive() {
            return Number::one();
        } else if self.is_negative() {
            return Number::neg_one();
        } else {
            return Number::zero();
        }
    }

    pub fn sqrt(&self) -> Number {
        Number::Float(self.to_f64().sqrt())
    }

    pub fn ln(&self) -> Number {
        Number::Float(self.to_f64().ln())
    }

    pub fn log(&self) -> Number {
        Number::Float(self.to_f64().log10())
    }

    pub fn cbrt(&self) -> Number {
        Number::Float(self.to_f64().cbrt())
    }

    pub fn sin(&self) -> Number {
        Number::Float(self.to_f64().sin())
    }

    pub fn cos(&self) -> Number {
        Number::Float(self.to_f64().cos())
    }

    pub fn tan(&self) -> Number {
        Number::Float(self.to_f64().tan())
    }

    pub fn asin(&self) -> Number {
        Number::Float(self.to_f64().asin())
    }

    pub fn acos(&self) -> Number {
        Number::Float(self.to_f64().acos())
    }

    pub fn atan(&self) -> Number {
        Number::Float(self.to_f64().atan())
    }

    pub fn atan2(&self, o: &Self) -> Number {
        Number::Float(self.to_f64().atan2(o.to_f64()))
    }

    pub fn sinh(&self) -> Number {
        Number::Float(self.to_f64().sinh())
    }

    pub fn cosh(&self) -> Number {
        Number::Float(self.to_f64().cosh())
    }

    pub fn tanh(&self) -> Number {
        Number::Float(self.to_f64().tanh())
    }

    pub fn asinh(&self) -> Number {
        Number::Float(self.to_f64().asinh())
    }

    pub fn acosh(&self) -> Number {
        Number::Float(self.to_f64().acosh())
    }

    pub fn atanh(&self) -> Number {
        Number::Float(self.to_f64().atanh())
    }
}

