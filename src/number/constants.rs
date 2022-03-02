
use std::ops::Neg;

use num::*;

use super::Number;

impl Number {
    pub fn zero() -> Number {
        Number::Rational(BigRational::zero())
    }

    pub fn one() -> Number {
        Number::Rational(BigRational::one())
    }

    pub fn neg_one() -> Number {
        Number::Rational(BigRational::one().neg())
    }

    pub fn pi() -> Number {
        Number::Float(3.1415926535897932384626433832795)
    }

    pub fn e() -> Number {
        Number::Float(2.7182818284590452353602874713527)
    }
}

