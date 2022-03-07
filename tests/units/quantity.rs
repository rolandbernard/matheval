
use std::{ops::*, str::FromStr};
use num::traits::Pow;

use matheval::{Quantity, Number, Unit, BaseUnit};

#[test]
fn from_str_integer() {
    let num = Quantity::from_str("123456789").expect("Failed parsing number");
    let exp = Quantity::unitless(Number::from_i64(123456789));
    assert_eq!(exp, num);
    let num = Quantity::from_str("-123456789").expect("Failed parsing number");
    let exp = Quantity::unitless(Number::from_i64(-123456789));
    assert_eq!(exp, num);
}

#[test]
fn from_str_binary_integer() {
    let num = Quantity::from_str("0b1010111100").expect("Failed parsing number");
    let exp = Quantity::unitless(Number::from_i64(0b1010111100));
    assert_eq!(exp, num);
    let num = Quantity::from_str("-0b1010111100").expect("Failed parsing number");
    let exp = Quantity::unitless(Number::from_i64(-0b1010111100));
    assert_eq!(exp, num);
}

#[test]
fn from_str_octal_integer() {
    let num = Quantity::from_str("0o75421603").expect("Failed parsing number");
    let exp = Quantity::unitless(Number::from_i64(0o75421603));
    assert_eq!(exp, num);
    let num = Quantity::from_str("-0o75421603").expect("Failed parsing number");
    let exp = Quantity::unitless(Number::from_i64(-0o75421603));
    assert_eq!(exp, num);
}

#[test]
fn from_str_hex_integer() {
    let num = Quantity::from_str("0xfea78").expect("Failed parsing number");
    let exp = Quantity::unitless(Number::from_i64(0xfea78));
    assert_eq!(exp, num);
    let num = Quantity::from_str("-0xfea78").expect("Failed parsing number");
    let exp = Quantity::unitless(Number::from_i64(-0xfea78));
    assert_eq!(exp, num);
}

#[test]
fn from_str_nonint() {
    let num = Quantity::from_str("1234.56789").expect("Failed parsing number");
    let exp = Quantity::unitless(Number::from_i64s(123456789, 100000));
    assert_eq!(exp, num);
    let num = Quantity::from_str("-1234.56789").expect("Failed parsing number");
    let exp = Quantity::unitless(Number::from_i64s(-123456789, 100000));
    assert_eq!(exp, num);
}

#[test]
fn from_str_binary_nonint() {
    let num = Quantity::from_str("0b1001.1").expect("Failed parsing number");
    let exp = Quantity::unitless(Number::from_i64s(19, 2));
    assert_eq!(exp, num);
    let num = Quantity::from_str("-0b1001.1").expect("Failed parsing number");
    let exp = Quantity::unitless(Number::from_i64s(-19, 2));
    assert_eq!(exp, num);
}

#[test]
fn from_str_octal_nonint() {
    let num = Quantity::from_str("0o740.2").expect("Failed parsing number");
    let exp = Quantity::unitless(Number::from_i64s(1921, 4));
    assert_eq!(exp, num);
    let num = Quantity::from_str("-0o740.2").expect("Failed parsing number");
    let exp = Quantity::unitless(Number::from_i64s(-1921, 4));
    assert_eq!(exp, num);
}

#[test]
fn from_str_hex_nonint() {
    let num = Quantity::from_str("0xa4f.f").expect("Failed parsing number");
    let exp = Quantity::unitless(Number::from_i64s(42239, 16));
    assert_eq!(exp, num);
    let num = Quantity::from_str("-0xa4f.f").expect("Failed parsing number");
    let exp = Quantity::unitless(Number::from_i64s(-42239, 16));
    assert_eq!(exp, num);
}

#[test]
fn from_str_exponent() {
    let num = Quantity::from_str("12.3452e2").expect("Failed parsing number");
    let exp = Quantity::unitless(Number::from_i64s(30863, 25));
    assert_eq!(exp, num);
    let num = Quantity::from_str("12.3452e+2").expect("Failed parsing number");
    let exp = Quantity::unitless(Number::from_i64s(30863, 25));
    assert_eq!(exp, num);
    let num = Quantity::from_str("12.3452e-2").expect("Failed parsing number");
    let exp = Quantity::unitless(Number::from_i64s(30863, 250000));
    assert_eq!(exp, num);
    let num = Quantity::from_str("-12.3452e2").expect("Failed parsing number");
    let exp = Quantity::unitless(Number::from_i64s(-30863, 25));
    assert_eq!(exp, num);
    let num = Quantity::from_str("-12.3452e+2").expect("Failed parsing number");
    let exp = Quantity::unitless(Number::from_i64s(-30863, 25));
    assert_eq!(exp, num);
    let num = Quantity::from_str("-12.3452e-2").expect("Failed parsing number");
    let exp = Quantity::unitless(Number::from_i64s(-30863, 250000));
    assert_eq!(exp, num);
}

#[test]
fn from_str_error() {
    assert!(Quantity::from_str("").is_err(), "Empty literals are illegal");
    assert!(Quantity::from_str("5.5.5").is_err(), "Two '.' characters");
    assert!(Quantity::from_str(".42").is_err(), "Missing integer part");
    assert!(Quantity::from_str("42.").is_err(), "Missing fractional part");
    assert!(Quantity::from_str("5e12e12").is_err(), "Two 'e' characters");
    assert!(Quantity::from_str("++5").is_err(), "Two sign characters");
    assert!(Quantity::from_str("--5").is_err(), "Two sign characters");
    assert!(Quantity::from_str("+-5").is_err(), "Two sign characters");
    assert!(Quantity::from_str("1b001").is_err(), "Base indicator after non-zero");
    assert!(Quantity::from_str("00x001").is_err(), "Base indicator after multiple zero");
    assert!(Quantity::from_str("-").is_err(), "Missing value after sign character");
    assert!(Quantity::from_str("+").is_err(), "Missing value after sign character");
    assert!(Quantity::from_str("+e12").is_err(), "Missing value after sign character");
    assert!(Quantity::from_str("+.12").is_err(), "Missing value after sign character");
    assert!(Quantity::from_str("+.").is_err(), "Missing value after sign character");
    assert!(Quantity::from_str("0x").is_err(), "Missing value after base indicator");
    assert!(Quantity::from_str("12e").is_err(), "Missing exponent number");
    assert!(Quantity::from_str("12e+").is_err(), "Missing exponent number");
    assert!(Quantity::from_str("12e-").is_err(), "Missing exponent number");
    assert!(Quantity::from_str("12e--12").is_err(), "Double exponent sign character");
    assert!(Quantity::from_str("12e++12").is_err(), "Double exponent sign character");
    assert!(Quantity::from_str("12e-+12").is_err(), "Double exponent sign character");
    assert!(Quantity::from_str("0b1e1").is_err(), "Exponent on binary");
    assert!(Quantity::from_str("0o1e1").is_err(), "Exponent on octal");
    assert!(Quantity::from_str("0o1e+1").is_err(), "Exponent on hex");
    assert!(Quantity::from_str("123+456").is_err(), "Sign character in center");
    assert!(Quantity::from_str("12abc567").is_err(), "Non digit character");
    assert!(Quantity::from_str("+abc").is_err(), "No digits");
    assert!(Quantity::from_str("123+").is_err(), "Unexpected sign character");
    assert!(Quantity::from_str("123m").is_err(), "Unexpected unit");
    assert!(Quantity::from_str("123kg").is_err(), "Unexpected unit");
}

#[test]
fn to_string_integer() {
    let num = Quantity::unitless(Number::from_i64(4200));
    assert_eq!("4200", num.to_string());
}

#[test]
fn to_string_fraction() {
    let num = Quantity::unitless(Number::from_i64s(42, 10));
    assert_eq!("21/5", num.to_string());
}

#[test]
fn to_string_float() {
    let num = Quantity::unitless(Number::Float(12.3456789));
    assert_eq!("12.3456789", num.to_string());
}

#[test]
fn to_string_with_unit() {
    let num = Quantity::new(
        Number::from_i64(4200),
        Unit::base(BaseUnit::Gram).mul(Unit::base(BaseUnit::Meter)).mul(Unit::base(BaseUnit::Second))
    );
    assert_eq!("4200 g m s", num.to_string());
}

#[test]
fn to_string_with_unit2() {
    let num = Quantity::new(
        Number::from_i64(4200),
        Unit::base(BaseUnit::Gram).pow(Number::from_i64(2))
            .mul(Unit::base(BaseUnit::Meter).pow(Number::from_i64(-10)))
            .mul(Unit::base(BaseUnit::Second).pow(Number::from_i64s(-3, 4)))
    );
    assert_eq!("4200 g^2 m^-10 s^(-3/4)", num.to_string());
}

#[test]
fn is_unitless() {
    assert!(Quantity::unitless(Number::Float(12.3456789)).is_unitless());
    assert!(Quantity::unitless(Number::from_i64(123)).is_unitless());
    assert!(Quantity::unitless(Number::from_i64s(123, 7)).is_unitless());
}

