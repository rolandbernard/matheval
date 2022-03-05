
use std::{str::FromStr, cmp::Ordering, ops::*};
use num::traits::Pow;
use num::*;

use matheval::Number;

#[test]
fn from_str_integer() {
    let num = Number::from_str("123456789").expect("Failed parsing number");
    let exp = Number::Rational(BigRational::new(BigInt::from_i32(123456789).unwrap(), BigInt::one()));
    assert_eq!(exp, num);
    let num = Number::from_str("-123456789").expect("Failed parsing number");
    let exp = Number::Rational(BigRational::new(BigInt::from_i32(-123456789).unwrap(), BigInt::one()));
    assert_eq!(exp, num);
}

#[test]
fn from_str_binary_integer() {
    let num = Number::from_str("0b1010111100").expect("Failed parsing number");
    let exp = Number::Rational(BigRational::new(BigInt::from_i32(700).unwrap(), BigInt::one()));
    assert_eq!(exp, num);
    let num = Number::from_str("-0b1010111100").expect("Failed parsing number");
    let exp = Number::Rational(BigRational::new(BigInt::from_i32(-700).unwrap(), BigInt::one()));
    assert_eq!(exp, num);
}

#[test]
fn from_str_octal_integer() {
    let num = Number::from_str("0o75421603").expect("Failed parsing number");
    let exp = Number::Rational(BigRational::new(BigInt::from_i32(16130947).unwrap(), BigInt::one()));
    assert_eq!(exp, num);
    let num = Number::from_str("-0o75421603").expect("Failed parsing number");
    let exp = Number::Rational(BigRational::new(BigInt::from_i32(-16130947).unwrap(), BigInt::one()));
    assert_eq!(exp, num);
}

#[test]
fn from_str_hex_integer() {
    let num = Number::from_str("0xfea78").expect("Failed parsing number");
    let exp = Number::Rational(BigRational::new(BigInt::from_i32(1043064).unwrap(), BigInt::one()));
    assert_eq!(exp, num);
    let num = Number::from_str("-0xfea78").expect("Failed parsing number");
    let exp = Number::Rational(BigRational::new(BigInt::from_i32(-1043064).unwrap(), BigInt::one()));
    assert_eq!(exp, num);
}

#[test]
fn from_str_nonint() {
    let num = Number::from_str("1234.56789").expect("Failed parsing number");
    let exp = Number::Rational(BigRational::new(
        BigInt::from_i32(123456789).unwrap(),
        BigInt::from_i32(100000).unwrap()
    ));
    assert_eq!(exp, num);
    let num = Number::from_str("-1234.56789").expect("Failed parsing number");
    let exp = Number::Rational(BigRational::new(
        BigInt::from_i32(-123456789).unwrap(),
        BigInt::from_i32(100000).unwrap()
    ));
    assert_eq!(exp, num);
}

#[test]
fn from_str_binary_nonint() {
    let num = Number::from_str("0b1001.1").expect("Failed parsing number");
    let exp = Number::Rational(BigRational::new(
        BigInt::from_i32(19).unwrap(),
        BigInt::from_i32(2).unwrap()
    ));
    assert_eq!(exp, num);
}

#[test]
fn from_str_octal_nonint() {
    let num = Number::from_str("0o740.2").expect("Failed parsing number");
    let exp = Number::Rational(BigRational::new(
        BigInt::from_i32(1921).unwrap(),
        BigInt::from_i32(4).unwrap()
    ));
    assert_eq!(exp, num);
}

#[test]
fn from_str_hex_nonint() {
    let num = Number::from_str("0xa4f.f").expect("Failed parsing number");
    let exp = Number::Rational(BigRational::new(
        BigInt::from_i32(42239).unwrap(),
        BigInt::from_i32(16).unwrap()
    ));
    assert_eq!(exp, num);
}

#[test]
fn from_str_exponent() {
    let num = Number::from_str("12.3452e2").expect("Failed parsing number");
    let exp = Number::Rational(BigRational::new(
        BigInt::from_i32(30863).unwrap(),
        BigInt::from_i32(25).unwrap()
    ));
    assert_eq!(exp, num);
    let num = Number::from_str("12.3452e+2").expect("Failed parsing number");
    let exp = Number::Rational(BigRational::new(
        BigInt::from_i32(30863).unwrap(),
        BigInt::from_i32(25).unwrap()
    ));
    assert_eq!(exp, num);
    let num = Number::from_str("12.3452e-2").expect("Failed parsing number");
    let exp = Number::Rational(BigRational::new(
        BigInt::from_i32(30863).unwrap(),
        BigInt::from_i32(250000).unwrap()
    ));
    assert_eq!(exp, num);
    let num = Number::from_str("-12.3452e2").expect("Failed parsing number");
    let exp = Number::Rational(BigRational::new(
        BigInt::from_i32(-30863).unwrap(),
        BigInt::from_i32(25).unwrap()
    ));
    assert_eq!(exp, num);
    let num = Number::from_str("+12.3452e2").expect("Failed parsing number");
    let exp = Number::Rational(BigRational::new(
        BigInt::from_i32(30863).unwrap(),
        BigInt::from_i32(25).unwrap()
    ));
    assert_eq!(exp, num);
}

#[test]
fn from_str_error() {
    assert!(Number::from_str("").is_err(), "Empty literals are illegal");
    assert!(Number::from_str("5.5.5").is_err(), "Two '.' characters");
    assert!(Number::from_str(".42").is_err(), "Missing integer part");
    assert!(Number::from_str("42.").is_err(), "Missing fractional part");
    assert!(Number::from_str("5e12e12").is_err(), "Two 'e' characters");
    assert!(Number::from_str("++5").is_err(), "Two sign characters");
    assert!(Number::from_str("--5").is_err(), "Two sign characters");
    assert!(Number::from_str("+-5").is_err(), "Two sign characters");
    assert!(Number::from_str("1b001").is_err(), "Base indicator after non-zero");
    assert!(Number::from_str("00x001").is_err(), "Base indicator after multiple zero");
    assert!(Number::from_str("-").is_err(), "Missing value after sign character");
    assert!(Number::from_str("+").is_err(), "Missing value after sign character");
    assert!(Number::from_str("+e12").is_err(), "Missing value after sign character");
    assert!(Number::from_str("+.12").is_err(), "Missing value after sign character");
    assert!(Number::from_str("+.").is_err(), "Missing value after sign character");
    assert!(Number::from_str("0x").is_err(), "Missing value after base indicator");
    assert!(Number::from_str("12e").is_err(), "Missing exponent number");
    assert!(Number::from_str("12e+").is_err(), "Missing exponent number");
    assert!(Number::from_str("12e-").is_err(), "Missing exponent number");
    assert!(Number::from_str("12e--12").is_err(), "Double exponent sign character");
    assert!(Number::from_str("12e++12").is_err(), "Double exponent sign character");
    assert!(Number::from_str("12e-+12").is_err(), "Double exponent sign character");
    assert!(Number::from_str("0b1e1").is_err(), "Exponent on binary");
    assert!(Number::from_str("0o1e1").is_err(), "Exponent on octal");
    assert!(Number::from_str("0o1e+1").is_err(), "Exponent on hex");
    assert!(Number::from_str("123+456").is_err(), "Sign character in center");
    assert!(Number::from_str("12abc567").is_err(), "Non digit character");
    assert!(Number::from_str("+abc").is_err(), "No digits");
    assert!(Number::from_str("123+").is_err(), "Unexpected sign character");
}

#[test]
fn to_string_integer() {
    let num = Number::from_str("004200").expect("Failed parsing number");
    assert_eq!("4200", num.to_string());
}

#[test]
fn to_string_fraction() {
    let num = Number::from_str("004.200").expect("Failed parsing number");
    assert_eq!("21/5", num.to_string());
}

#[test]
fn to_string_float() {
    let num = Number::Float(12.3456789);
    assert_eq!("12.3456789", num.to_string());
}

#[test]
fn to_f64_integer() {
    let num = Number::from_str("004200").expect("Failed parsing number");
    assert_eq!(4200.0, num.to_f64());
}

#[test]
fn to_f64_fraction() {
    let num = Number::from_str("004.200").expect("Failed parsing number");
    assert_eq!(4.2, num.to_f64());
}

#[test]
fn to_f64_float() {
    let num = Number::Float(12.3456789);
    assert_eq!(12.3456789, num.to_f64());
}

#[test]
fn is_integer_integer() {
    let num = Number::from_str("004200").expect("Failed parsing number");
    assert!(num.is_integer());
}

#[test]
fn is_integer_fraction() {
    let num = Number::from_str("004.200").expect("Failed parsing number");
    assert!(!num.is_integer());
}

#[test]
fn is_integer_float() {
    let num = Number::Float(12.3456789);
    assert!(!num.is_integer());
    let num = Number::Float(123456789.0);
    assert!(num.is_integer());
}

#[test]
fn is_zero_rational() {
    let num = Number::from_str("0.02e-100").expect("Failed parsing number");
    assert!(!num.is_zero());
    let num = Number::from_str("-0.02e-100").expect("Failed parsing number");
    assert!(!num.is_zero());
    let num = Number::from_str("0.00000").expect("Failed parsing number");
    assert!(num.is_zero());
}

#[test]
fn is_zero_float() {
    let num = Number::Float(12.3456789);
    assert!(!num.is_zero());
    let num = Number::Float(-12.3456789);
    assert!(!num.is_zero());
    let num = Number::Float(f64::INFINITY);
    assert!(!num.is_zero());
    let num = Number::Float(f64::NEG_INFINITY);
    assert!(!num.is_zero());
    let num = Number::Float(f64::NAN);
    assert!(!num.is_zero());
    let num = Number::Float(0.0);
    assert!(num.is_zero());
}

#[test]
fn is_positive_rational() {
    let num = Number::from_str("0.02e-100").expect("Failed parsing number");
    assert!(num.is_positive());
    let num = Number::from_str("-0.02e-100").expect("Failed parsing number");
    assert!(!num.is_positive());
    let num = Number::from_str("0.00000").expect("Failed parsing number");
    assert!(!num.is_positive());
}

#[test]
fn is_positive_float() {
    let num = Number::Float(12.3456789);
    assert!(num.is_positive());
    let num = Number::Float(f64::INFINITY);
    assert!(num.is_positive());
    let num = Number::Float(-12.3456789);
    assert!(!num.is_positive());
    let num = Number::Float(f64::NEG_INFINITY);
    assert!(!num.is_positive());
    let num = Number::Float(f64::NAN);
    assert!(!num.is_positive());
    let num = Number::Float(0.0);
    assert!(!num.is_positive());
}

#[test]
fn is_negative_rational() {
    let num = Number::from_str("0.02e-100").expect("Failed parsing number");
    assert!(!num.is_negative());
    let num = Number::from_str("-0.02e-100").expect("Failed parsing number");
    assert!(num.is_negative());
    let num = Number::from_str("0.00000").expect("Failed parsing number");
    assert!(!num.is_negative());
}

#[test]
fn is_negative_float() {
    let num = Number::Float(12.3456789);
    assert!(!num.is_negative());
    let num = Number::Float(f64::INFINITY);
    assert!(!num.is_negative());
    let num = Number::Float(-12.3456789);
    assert!(num.is_negative());
    let num = Number::Float(f64::NEG_INFINITY);
    assert!(num.is_negative());
    let num = Number::Float(f64::NAN);
    assert!(!num.is_negative());
    let num = Number::Float(0.0);
    assert!(!num.is_negative());
}

#[test]
fn eq_rational() {
    assert!(Number::Rational(BigRational::new(
        BigInt::from_i32(42239).unwrap(),
        BigInt::from_i32(16).unwrap()
    )).eq(&Number::Rational(BigRational::new(
        BigInt::from_i32(42239).unwrap(),
        BigInt::from_i32(16).unwrap()
    ))));
    assert!(Number::Rational(BigRational::new(
        BigInt::from_i32(-4356).unwrap(),
        BigInt::from_i32(123456789).unwrap()
    )).eq(&Number::Rational(BigRational::new(
        BigInt::from_i32(-4356).unwrap(),
        BigInt::from_i32(123456789).unwrap()
    ))));
    assert!(!Number::Rational(BigRational::new(
        BigInt::from_i32(-4356).unwrap(),
        BigInt::from_i32(123456789).unwrap()
    )).eq(&Number::Rational(BigRational::new(
        BigInt::from_i32(4356).unwrap(),
        BigInt::from_i32(123456789).unwrap()
    ))));
    assert!(!Number::Rational(BigRational::new(
        BigInt::from_i32(4356).unwrap(),
        BigInt::from_i32(123456789).unwrap()
    )).eq(&Number::Rational(BigRational::new(
        BigInt::from_i32(4356).unwrap(),
        BigInt::from_i32(123456788).unwrap()
    ))));
}

#[test]
fn eq_float() {
    assert!(Number::Float(42.12).eq(&Number::Float(42.12)));
    assert!(Number::Float(0.012e-199).eq(&Number::Float(0.12e-200)));
    assert!(Number::Float(0.12e+199).eq(&Number::Float(0.012e+200)));
    assert!(Number::Float(f64::INFINITY).eq(&Number::Float(f64::INFINITY)));
    assert!(Number::Float(f64::NEG_INFINITY).eq(&Number::Float(f64::NEG_INFINITY)));
    assert!(Number::Float(-0.0).eq(&Number::Float(0.0)));
    assert!(!Number::Float(42.12).eq(&Number::Float(42.121)));
    assert!(!Number::Float(-42.12).eq(&Number::Float(42.12)));
    assert!(!Number::Float(f64::NAN).eq(&Number::Float(f64::NAN)));
    assert!(!Number::Float(1.0).eq(&Number::Float(f64::NAN)));
    assert!(!Number::Float(f64::NAN).eq(&Number::Float(1.0)));
    assert!(!Number::Float(f64::INFINITY).eq(&Number::Float(f64::NEG_INFINITY)));
}

#[test]
fn eq_float_rational() {
    assert!(Number::Float(42.25).eq(&Number::from_str("42.25").expect("Failed parsing number")));
    assert!(Number::Float(-42.25).eq(&Number::from_str("-42.25").expect("Failed parsing number")));
    assert!(Number::Float(-0.0).eq(&Number::from_str("0.0").expect("Failed parsing number")));
    assert!(Number::Float(0.0).eq(&Number::from_str("-0.0").expect("Failed parsing number")));
    assert!(Number::Float(1e10).eq(&Number::from_str("1e10").expect("Failed parsing number")));
    assert!(!Number::Float(0.1).eq(&Number::from_str("0.1").expect("Failed parsing number")));
    assert!(!Number::Float(0.2e-100).eq(&Number::from_str("0.0").expect("Failed parsing number")));
    assert!(!Number::Float(f64::NAN).eq(&Number::from_str("0.0").expect("Failed parsing number")));
    assert!(!Number::Float(1e100).eq(&Number::from_str("1e-100").expect("Failed parsing number")));
}

#[test]
fn ord_rational() {
    assert_eq!(Number::Rational(BigRational::new(
        BigInt::from_i32(42239).unwrap(),
        BigInt::from_i32(16).unwrap()
    )).partial_cmp(&Number::Rational(BigRational::new(
        BigInt::from_i32(42239).unwrap(),
        BigInt::from_i32(16).unwrap()
    ))), Some(Ordering::Equal));
    assert_eq!(Number::Rational(BigRational::new(
        BigInt::from_i32(-4356).unwrap(),
        BigInt::from_i32(123456789).unwrap()
    )).partial_cmp(&Number::Rational(BigRational::new(
        BigInt::from_i32(-4356).unwrap(),
        BigInt::from_i32(123456789).unwrap()
    ))), Some(Ordering::Equal));
    assert_eq!(Number::Rational(BigRational::new(
        BigInt::from_i32(-4356).unwrap(),
        BigInt::from_i32(123456789).unwrap()
    )).partial_cmp(&Number::Rational(BigRational::new(
        BigInt::from_i32(4356).unwrap(),
        BigInt::from_i32(123456789).unwrap()
    ))), Some(Ordering::Less));
    assert_eq!(Number::Rational(BigRational::new(
        BigInt::from_i32(4356).unwrap(),
        BigInt::from_i32(123456788).unwrap()
    )).partial_cmp(&Number::Rational(BigRational::new(
        BigInt::from_i32(4356).unwrap(),
        BigInt::from_i32(123456789).unwrap()
    ))), Some(Ordering::Greater));
}

#[test]
fn ord_float() {
    assert!(Number::Float(42.12) == Number::Float(42.12));
    assert!(Number::Float(0.012e-199) == Number::Float(0.12e-200));
    assert!(Number::Float(0.12e+199) == Number::Float(0.012e+200));
    assert!(Number::Float(f64::INFINITY) == Number::Float(f64::INFINITY));
    assert!(Number::Float(f64::NEG_INFINITY) == Number::Float(f64::NEG_INFINITY));
    assert!(Number::Float(-0.0) == Number::Float(0.0));
    assert!(Number::Float(42.12) < Number::Float(42.121));
    assert!(Number::Float(-42.12) < Number::Float(42.12));
    assert!(Number::Float(43.12) > Number::Float(42.121));
    assert!(Number::Float(-42.12) > Number::Float(-142.12));
    assert!(Number::Float(f64::INFINITY) > Number::Float(f64::NEG_INFINITY));
}

#[test]
fn ord_float_rational() {
    assert!(Number::Float(42.25) == Number::from_str("42.25").expect("Failed parsing number"));
    assert!(Number::Float(-42.25) == Number::from_str("-42.25").expect("Failed parsing number"));
    assert!(Number::Float(-0.0) == Number::from_str("0.0").expect("Failed parsing number"));
    assert!(Number::Float(0.0) == Number::from_str("-0.0").expect("Failed parsing number"));
    assert!(Number::Float(0.1) < Number::from_str("0.12").expect("Failed parsing number"));
    assert!(Number::Float(0.2e-100) > Number::from_str("0.0").expect("Failed parsing number"));
    assert!(Number::Float(1e100) > Number::from_str("1e-100").expect("Failed parsing number"));
    assert!(Number::Float(1e-100) < Number::from_str("1e100").expect("Failed parsing number"));
    assert!(Number::Float(1e-100) > Number::from_str("-1e100").expect("Failed parsing number"));
}

#[test]
fn neg_rational() {
    assert_eq!(Number::from_str("0.02e-100").unwrap().neg().unwrap(), Number::from_str("-0.02e-100").unwrap());
    assert_eq!(Number::from_str("-0.02e-100").unwrap().neg().unwrap(), Number::from_str("0.02e-100").unwrap());
    assert_eq!(Number::from_str("42.12").unwrap().neg().unwrap(), Number::from_str("-42.12").unwrap());
    assert_eq!(Number::from_str("-42.12").unwrap().neg().unwrap(), Number::from_str("42.12").unwrap());
}

#[test]
fn neg_float() {
    assert_eq!(Number::Float(12.3456789).neg().unwrap(), Number::Float(-12.3456789));
    assert_eq!(Number::Float(42.0).neg().unwrap(), Number::Float(-42.0));
    assert_eq!(Number::Float(-42.12).neg().unwrap(), Number::Float(42.12));
    assert_eq!(Number::Float(1e100).neg().unwrap(), Number::Float(-1e100));
    assert_eq!(Number::Float(-1e100).neg().unwrap(), Number::Float(1e100));
}

#[test]
fn add_rational() {
    assert_eq!(
        Number::from_str("0.02e-100").unwrap().add(
            Number::from_str("-0.02e-100").unwrap()
        ).unwrap(),
        Number::from_str("0.0").unwrap()
    );
    assert_eq!(
        Number::from_str("0.01e-100").unwrap().add(
            Number::from_str("0.01e-100").unwrap()
        ).unwrap(),
        Number::from_str("0.02e-100").unwrap()
    );
    assert_eq!(
        Number::from_str("0.0").unwrap().add(
            Number::from_str("-42.12").unwrap()
        ).unwrap(),
        Number::from_str("-42.12").unwrap()
    );
    assert_eq!(
        Number::from_str("31.06").unwrap().add(
            Number::from_str("11.06").unwrap()
        ).unwrap(),
        Number::from_str("42.12").unwrap()
    );
}

#[test]
fn add_float() {
    assert_eq!(
        Number::Float(12.3456789).add(
            Number::Float(-12.3456789)
        ).unwrap(),
        Number::Float(0.0)
    );
    assert_eq!(
        Number::Float(0.0).add(
            Number::Float(-42.0)
        ).unwrap(),
        Number::Float(-42.0)
    );
    assert_eq!(
        Number::Float(12.12).add(
            Number::Float(30.0)
        ).unwrap(),
        Number::Float(42.12)
    );
    assert_eq!(
        Number::Float(-5e99).add(
            Number::Float(-5e99)
        ).unwrap(),
        Number::Float(-1e100)
    );
    assert_eq!(
        Number::Float(0.25e100).add(
            Number::Float(0.75e100)
        ).unwrap(),
        Number::Float(1e100)
    );
}

#[test]
fn sub_rational() {
    assert_eq!(
        Number::from_str("0.02e-100").unwrap().sub(
            Number::from_str("0.02e-100").unwrap()
        ).unwrap(),
        Number::from_str("0.0").unwrap()
    );
    assert_eq!(
        Number::from_str("0.01e-100").unwrap().sub(
            Number::from_str("-0.01e-100").unwrap()
        ).unwrap(),
        Number::from_str("0.02e-100").unwrap()
    );
    assert_eq!(
        Number::from_str("0.0").unwrap().sub(
            Number::from_str("42.12").unwrap()
        ).unwrap(),
        Number::from_str("-42.12").unwrap()
    );
    assert_eq!(
        Number::from_str("31.06").unwrap().sub(
            Number::from_str("-11.06").unwrap()
        ).unwrap(),
        Number::from_str("42.12").unwrap()
    );
}

#[test]
fn sub_float() {
    assert_eq!(
        Number::Float(12.3456789).sub(
            Number::Float(12.3456789)
        ).unwrap(),
        Number::Float(0.0)
    );
    assert_eq!(
        Number::Float(0.0).sub(
            Number::Float(42.0)
        ).unwrap(),
        Number::Float(-42.0)
    );
    assert_eq!(
        Number::Float(12.12).sub(
            Number::Float(-30.0)
        ).unwrap(),
        Number::Float(42.12)
    );
    assert_eq!(
        Number::Float(-5e99).sub(
            Number::Float(5e99)
        ).unwrap(),
        Number::Float(-1e100)
    );
    assert_eq!(
        Number::Float(0.25e100).sub(
            Number::Float(-0.75e100)
        ).unwrap(),
        Number::Float(1e100)
    );
}

#[test]
fn mul_rational() {
    assert_eq!(
        Number::from_str("0.02e-100").unwrap().mul(
            Number::from_str("5").unwrap()
        ).unwrap(),
        Number::from_str("0.1e-100").unwrap()
    );
    assert_eq!(
        Number::from_str("0.01e-10").unwrap().mul(
            Number::from_str("-1e-10").unwrap()
        ).unwrap(),
        Number::from_str("-0.01e-20").unwrap()
    );
    assert_eq!(
        Number::from_str("0.0").unwrap().mul(
            Number::from_str("42.12").unwrap()
        ).unwrap(),
        Number::from_str("0.0").unwrap()
    );
    assert_eq!(
        Number::from_str("1.1").unwrap().mul(
            Number::from_str("1.2").unwrap()
        ).unwrap(),
        Number::from_str("1.32").unwrap()
    );
}

#[test]
fn mul_float() {
    assert_eq!(
        Number::Float(12.3456789).mul(
            Number::Float(0.0)
        ).unwrap(),
        Number::Float(0.0)
    );
    assert_eq!(
        Number::Float(-1.0).mul(
            Number::Float(42.0)
        ).unwrap(),
        Number::Float(-42.0)
    );
    assert_eq!(
        Number::Float(1.25).mul(
            Number::Float(1.75)
        ).unwrap(),
        Number::Float(2.1875)
    );
    assert_eq!(
        Number::Float(-5e99).mul(
            Number::Float(-1.0)
        ).unwrap(),
        Number::Float(5e99)
    );
    assert_eq!(
        Number::Float(1.5).mul(
            Number::Float(25.0)
        ).unwrap(),
        Number::Float(37.5)
    );
}

#[test]
fn div_rational() {
    assert_eq!(
        Number::from_str("42.12").unwrap().div(
            Number::from_str("42.12").unwrap()
        ).unwrap(),
        Number::from_str("1").unwrap()
    );
    assert_eq!(
        Number::from_str("42").unwrap().div(
            Number::from_str("-1e-10").unwrap()
        ).unwrap(),
        Number::from_str("-42e10").unwrap()
    );
    assert_eq!(
        Number::from_str("0.0").unwrap().div(
            Number::from_str("42.12").unwrap()
        ).unwrap(),
        Number::from_str("0.0").unwrap()
    );
    assert_eq!(
        Number::from_str("1.1").unwrap().div(
            Number::from_str("2.2").unwrap()
        ).unwrap(),
        Number::from_str("0.5").unwrap()
    );
    assert!(
        Number::from_str("1.1").unwrap().div(
            Number::from_str("0.0").unwrap()
        ).is_err()
    );
}

#[test]
fn div_float() {
    assert_eq!(
        Number::Float(0.0).div(
            Number::Float(12.42)
        ).unwrap(),
        Number::Float(0.0)
    );
    assert_eq!(
        Number::Float(-42.0).div(
            Number::Float(2.0)
        ).unwrap(),
        Number::Float(-21.0)
    );
    assert_eq!(
        Number::Float(13.5).div(
            Number::Float(1.5)
        ).unwrap(),
        Number::Float(9.0)
    );
    assert!(
        Number::Float(1.5).div(
            Number::Float(0.0)
        ).is_err()
    );
}

#[test]
fn pow_rational() {
    assert_eq!(
        Number::from_str("3").unwrap().pow(
            Number::from_str("16").unwrap()
        ).unwrap(),
        Number::from_str("43046721").unwrap()
    );
    assert_eq!(
        Number::from_str("0.5").unwrap().pow(
            Number::from_str("8").unwrap()
        ).unwrap(),
        Number::from_str("0.00390625").unwrap()
    );
    assert_eq!(
        Number::from_str("0.0").unwrap().pow(
            Number::from_str("42").unwrap()
        ).unwrap(),
        Number::from_str("0.0").unwrap()
    );
    assert!(
        Number::from_str("0.0").unwrap().pow(
            Number::from_str("-2").unwrap()
        ).is_err(),
    );
    assert!(
        Number::from_str("0").unwrap().pow(
            Number::from_str("0").unwrap()
        ).is_err(),
    );
}

#[test]
fn pow_float() {
    assert_eq!(
        Number::Float(0.0).pow(
            Number::Float(12.42)
        ).unwrap(),
        Number::Float(0.0)
    );
    assert_eq!(
        Number::Float(2.5).pow(
            Number::Float(2.5)
        ).unwrap(),
        Number::Float(9.882117688026186)
    );
    assert_eq!(
        Number::Float(3.0).pow(
            Number::Float(2.0)
        ).unwrap(),
        Number::Float(9.0)
    );
    assert!(
        Number::Float(0.0).pow(
            Number::Float(-2.0)
        ).is_err()
    );
    assert!(
        Number::Float(0.0).pow(
            Number::Float(0.0)
        ).is_err()
    );
}

