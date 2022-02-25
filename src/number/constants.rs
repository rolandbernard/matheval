
use num::*;

use super::Number;

impl Number {
    pub fn zero() -> Number {
        Number::Rational(BigRational::zero())
    }

    pub fn pi() -> Number {
        Number::Rational(BigRational::new(
            BigInt::from_u64(2646693125139304345).unwrap(), 
            BigInt::from_u64(842468587426513207).unwrap()
        ))
    }

    pub fn e() -> Number {
        Number::Rational(BigRational::new(
            BigInt::from_u128(27182818284590452353602874713527).unwrap(), 
            BigInt::from_u128(1000000000000000000000000000000).unwrap()
        ))
    }
}

