
use std::str::FromStr;
use num::*;

use matheval::Number;

#[test]
fn number_from_str_integer() {
    let num = Number::from_str("123456789").expect("Failed parsing number");
    let exp = Number::Rational(BigRational::new(BigInt::from_i32(123456789).unwrap(), BigInt::one()));
    assert_eq!(exp, num);
    let num = Number::from_str("-123456789").expect("Failed parsing number");
    let exp = Number::Rational(BigRational::new(BigInt::from_i32(-123456789).unwrap(), BigInt::one()));
    assert_eq!(exp, num);
}

#[test]
fn number_from_str_binary_integer() {
    let num = Number::from_str("0b1010111100").expect("Failed parsing number");
    let exp = Number::Rational(BigRational::new(BigInt::from_i32(700).unwrap(), BigInt::one()));
    assert_eq!(exp, num);
    let num = Number::from_str("-0b1010111100").expect("Failed parsing number");
    let exp = Number::Rational(BigRational::new(BigInt::from_i32(-700).unwrap(), BigInt::one()));
    assert_eq!(exp, num);
}

#[test]
fn number_from_str_octal_integer() {
    let num = Number::from_str("0o75421603").expect("Failed parsing number");
    let exp = Number::Rational(BigRational::new(BigInt::from_i32(16130947).unwrap(), BigInt::one()));
    assert_eq!(exp, num);
    let num = Number::from_str("-0o75421603").expect("Failed parsing number");
    let exp = Number::Rational(BigRational::new(BigInt::from_i32(-16130947).unwrap(), BigInt::one()));
    assert_eq!(exp, num);
}

#[test]
fn number_from_str_hex_integer() {
    let num = Number::from_str("0xfea78").expect("Failed parsing number");
    let exp = Number::Rational(BigRational::new(BigInt::from_i32(1043064).unwrap(), BigInt::one()));
    assert_eq!(exp, num);
    let num = Number::from_str("-0xfea78").expect("Failed parsing number");
    let exp = Number::Rational(BigRational::new(BigInt::from_i32(-1043064).unwrap(), BigInt::one()));
    assert_eq!(exp, num);
}

#[test]
fn number_from_str_nonint() {
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
fn number_from_str_binary_nonint() {
    let num = Number::from_str("0b1001.1").expect("Failed parsing number");
    let exp = Number::Rational(BigRational::new(
        BigInt::from_i32(19).unwrap(),
        BigInt::from_i32(2).unwrap()
    ));
    assert_eq!(exp, num);
}

#[test]
fn number_from_str_octal_nonint() {
    let num = Number::from_str("0o740.2").expect("Failed parsing number");
    let exp = Number::Rational(BigRational::new(
        BigInt::from_i32(1921).unwrap(),
        BigInt::from_i32(4).unwrap()
    ));
    assert_eq!(exp, num);
}

#[test]
fn number_from_str_hex_nonint() {
    let num = Number::from_str("0xa4f.f").expect("Failed parsing number");
    let exp = Number::Rational(BigRational::new(
        BigInt::from_i32(42239).unwrap(),
        BigInt::from_i32(16).unwrap()
    ));
    assert_eq!(exp, num);
}

#[test]
fn number_from_str_exponent() {
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
fn number_from_str_error() {
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
fn number_to_string_integer() {
    let num = Number::from_str("004200").expect("Failed parsing number");
    assert_eq!("4200", num.to_string());
}

#[test]
fn number_to_string_fraction() {
    let num = Number::from_str("004.200").expect("Failed parsing number");
    assert_eq!("21/5", num.to_string());
}

#[test]
fn number_to_string_float() {
    let num = Number::Float(12.3456789);
    assert_eq!("12.3456789", num.to_string());
}

#[test]
fn number_to_f64_integer() {
    let num = Number::from_str("004200").expect("Failed parsing number");
    assert_eq!(4200.0, num.to_f64());
}

#[test]
fn number_to_f64_fraction() {
    let num = Number::from_str("004.200").expect("Failed parsing number");
    assert_eq!(4.2, num.to_f64());
}

#[test]
fn number_to_f64_float() {
    let num = Number::Float(12.3456789);
    assert_eq!(12.3456789, num.to_f64());
}

#[test]
fn number_is_integer_integer() {
    let num = Number::from_str("004200").expect("Failed parsing number");
    assert!(num.is_integer());
}

#[test]
fn number_is_integer_fraction() {
    let num = Number::from_str("004.200").expect("Failed parsing number");
    assert!(!num.is_integer());
}

#[test]
fn number_is_integer_float() {
    let num = Number::Float(12.3456789);
    assert!(!num.is_integer());
    let num = Number::Float(123456789.0);
    assert!(num.is_integer());
}

#[test]
fn number_is_zero_rational() {
    let num = Number::from_str("0.02e-100").expect("Failed parsing number");
    assert!(!num.is_zero());
    let num = Number::from_str("-0.02e-100").expect("Failed parsing number");
    assert!(!num.is_zero());
    let num = Number::from_str("0.00000").expect("Failed parsing number");
    assert!(num.is_zero());
}

#[test]
fn number_is_zero_float() {
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
fn number_is_positive_rational() {
    let num = Number::from_str("0.02e-100").expect("Failed parsing number");
    assert!(num.is_positive());
    let num = Number::from_str("-0.02e-100").expect("Failed parsing number");
    assert!(!num.is_positive());
    let num = Number::from_str("0.00000").expect("Failed parsing number");
    assert!(!num.is_positive());
}

#[test]
fn number_is_positive_float() {
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
fn number_is_negative_rational() {
    let num = Number::from_str("0.02e-100").expect("Failed parsing number");
    assert!(!num.is_negative());
    let num = Number::from_str("-0.02e-100").expect("Failed parsing number");
    assert!(num.is_negative());
    let num = Number::from_str("0.00000").expect("Failed parsing number");
    assert!(!num.is_negative());
}

#[test]
fn number_is_negative_float() {
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

