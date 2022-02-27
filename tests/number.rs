
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
}


