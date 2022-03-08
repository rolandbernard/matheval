
use std::{ops::*, str::FromStr, cmp::Ordering};
use num::traits::Pow;

use matheval::{Quantity, Number, Unit, BaseUnit, QuantityContext};

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

#[test]
fn is_not_unitless() {
    assert!(!Quantity::new(Number::Float(12.3456789), Unit::base(BaseUnit::Gram)).is_unitless());
    assert!(!Quantity::new(Number::from_i64(123), Unit::base(BaseUnit::Gram)).is_unitless());
    assert!(!Quantity::new(Number::from_i64s(123, 7), Unit::base(BaseUnit::Gram)).is_unitless());
}

#[test]
fn coefficient() {
    assert_eq!(
        &Number::Float(12.3456789),
        Quantity::new(Number::Float(12.3456789), Unit::base(BaseUnit::Gram)).coefficient()
    );
    assert_eq!(
        &Number::from_i64(123),
        Quantity::new(Number::from_i64(123), Unit::base(BaseUnit::Gram)).coefficient()
    );
    assert_eq!(
        &Number::from_i64s(123, 7),
        Quantity::new(Number::from_i64s(123, 7), Unit::base(BaseUnit::Gram)).coefficient()
    );
}

#[test]
fn unit() {
    assert_eq!(
        &Unit::base(BaseUnit::Gram),
        Quantity::new(Number::Float(12.3456789), Unit::base(BaseUnit::Gram)).unit()
    );
    assert_eq!(
        &Unit::base(BaseUnit::Second).mul(Unit::base(BaseUnit::Meter).pow(Number::from_i64(10))),
        Quantity::new(Number::from_i64(123),
            Unit::base(BaseUnit::Second).mul(Unit::base(BaseUnit::Meter).pow(Number::from_i64(10)))
        ).unit()
    );
    assert_eq!(
        &Unit::empty(), Quantity::unitless(Number::from_i64s(123, 7)).unit()
    );
}

#[test]
fn abs() {
    assert_eq!(
        Quantity::new(Number::Float(12.3456789), Unit::base(BaseUnit::Gram)),
        Quantity::new(Number::Float(12.3456789), Unit::base(BaseUnit::Gram)).abs()
    );
    assert_eq!(
        Quantity::new(Number::Float(12.3456789), Unit::base(BaseUnit::Second)),
        Quantity::new(Number::Float(-12.3456789), Unit::base(BaseUnit::Second)).abs()
    );
    assert_eq!(
        Quantity::new(Number::from_i64(12), Unit::base(BaseUnit::Ampere)),
        Quantity::new(Number::from_i64(12), Unit::base(BaseUnit::Ampere)).abs()
    );
    assert_eq!(
        Quantity::new(Number::from_i64(12), Unit::base(BaseUnit::Candela)),
        Quantity::new(Number::from_i64(-12), Unit::base(BaseUnit::Candela)).abs()
    );
    assert_eq!(
        Quantity::new(Number::from_i64s(12, 7), Unit::base(BaseUnit::Kelvin)),
        Quantity::new(Number::from_i64s(12, 7), Unit::base(BaseUnit::Kelvin)).abs()
    );
    assert_eq!(
        Quantity::new(Number::from_i64s(12, 7), Unit::base(BaseUnit::Meter)),
        Quantity::new(Number::from_i64s(-12, 7), Unit::base(BaseUnit::Meter)).abs()
    );
}

#[test]
fn sign() {
    assert_eq!(
        Quantity::unitless(Number::one()),
        Quantity::new(Number::Float(12.3456789), Unit::base(BaseUnit::Gram)).sign()
    );
    assert_eq!(
        Quantity::unitless(Number::neg_one()),
        Quantity::new(Number::Float(-12.3456789), Unit::base(BaseUnit::Second)).sign()
    );
    assert_eq!(
        Quantity::unitless(Number::zero()),
        Quantity::new(Number::from_i64(0), Unit::base(BaseUnit::Ampere)).sign()
    );
    assert_eq!(
        Quantity::unitless(Number::one()),
        Quantity::new(Number::from_i64s(12, 7), Unit::base(BaseUnit::Kelvin)).sign()
    );
    assert_eq!(
        Quantity::unitless(Number::neg_one()),
        Quantity::new(Number::from_i64s(-12, 7), Unit::base(BaseUnit::Meter)).sign()
    );
}

#[test]
fn sqrt() {
    assert_eq!(
        Quantity::new(Number::from_i64s(8, 3), Unit::base(BaseUnit::Gram).pow(Number::from_i64s(1, 2))),
        Quantity::new(Number::from_i64s(64, 9), Unit::base(BaseUnit::Gram)).sqrt()
    );
    assert_eq!(
        Quantity::new(Number::from_i64s(3, 8), Unit::base(BaseUnit::Second)),
        Quantity::new(Number::from_i64s(9, 64), Unit::base(BaseUnit::Second).pow(Number::from_i64(2))).sqrt()
    );
}

#[test]
fn cbrt() {
    assert_eq!(
        Quantity::new(Number::from_i64s(8, 3), Unit::base(BaseUnit::Gram).pow(Number::from_i64s(1, 3))),
        Quantity::new(Number::from_i64s(512, 27), Unit::base(BaseUnit::Gram)).cbrt()
    );
    assert_eq!(
        Quantity::new(Number::from_i64s(3, 8), Unit::base(BaseUnit::Second)),
        Quantity::new(Number::from_i64s(27, 512), Unit::base(BaseUnit::Second).pow(Number::from_i64(3))).cbrt()
    );
}

#[test]
fn convert_to() {
    let context = QuantityContext::new();
    assert_eq!(
        Some(Number::from_i64(1000)), Quantity::new(Number::one(), Unit::base(BaseUnit::Gram)).convert_to_in("mg", &context)
    );
    assert_eq!(
        Some(Number::from_i64s(1, 1000)), Quantity::new(Number::one(), Unit::base(BaseUnit::Gram)).convert_to_in("kg", &context)
    );
    assert_eq!(
        Some(Number::from_i128s(5000000000000000000000000000, 801088317)),
        Quantity::new(Number::from_i64(1000),
            Unit::base(BaseUnit::Gram)
                .mul(Unit::base(BaseUnit::Meter).pow(Number::from_i64(2)))
                .mul(Unit::base(BaseUnit::Second).pow(Number::from_i64(-2)))
        ).convert_to_in("eV", &context)
    );
}

#[test]
fn ord() {
    assert_eq!(None,
        Quantity::new(Number::from_i64(12), Unit::base(BaseUnit::Second))
        .partial_cmp(&Quantity::unitless(Number::from_i64(12)))
    );
    assert_eq!(Some(Ordering::Less),
        Quantity::new(Number::from_i64(11), Unit::base(BaseUnit::Second))
        .partial_cmp(&Quantity::new(Number::from_i64(12), Unit::base(BaseUnit::Second)))
    );
    assert_eq!(Some(Ordering::Equal),
        Quantity::new(Number::from_i64(12), Unit::base(BaseUnit::Second))
        .partial_cmp(&Quantity::new(Number::from_i64(12), Unit::base(BaseUnit::Second)))
    );
    assert_eq!(Some(Ordering::Greater),
        Quantity::new(Number::from_i64(13), Unit::base(BaseUnit::Second))
        .partial_cmp(&Quantity::new(Number::from_i64(12), Unit::base(BaseUnit::Second)))
    );
}

#[test]
fn neg() {
    assert_eq!(
        Quantity::new(Number::from_i64s(8, 3), Unit::base(BaseUnit::Gram)),
        Quantity::new(Number::from_i64s(-8, 3), Unit::base(BaseUnit::Gram)).neg().expect("neg failed")
    );
    assert_eq!(
        Quantity::new(Number::from_i64s(-3, 8), Unit::base(BaseUnit::Second).pow(Number::from_i64(2))),
        Quantity::new(Number::from_i64s(3, 8), Unit::base(BaseUnit::Second).pow(Number::from_i64(2))).neg().expect("neg failed")
    );
}

#[test]
fn add() {
    assert_eq!(
        Quantity::new(Number::from_i64s(16, 3), Unit::base(BaseUnit::Gram)),
        Quantity::new(Number::from_i64s(8, 3), Unit::base(BaseUnit::Gram)).add(
            Quantity::new(Number::from_i64s(8, 3), Unit::base(BaseUnit::Gram)),
        ).expect("neg failed")
    );
    assert_eq!(
        Quantity::new(Number::zero(), Unit::base(BaseUnit::Second)),
        Quantity::new(Number::from_i64s(-8, 3), Unit::base(BaseUnit::Second)).add(
            Quantity::new(Number::from_i64s(8, 3), Unit::base(BaseUnit::Second)),
        ).expect("neg failed")
    );
}

#[test]
fn add_error() {
    assert!(
        Quantity::new(Number::from_i64s(8, 3), Unit::base(BaseUnit::Gram)).add(
            Quantity::new(Number::from_i64s(8, 3), Unit::base(BaseUnit::Second)),
        ).is_err()
    );
    assert!(
        Quantity::new(Number::from_i64s(8, 3), Unit::base(BaseUnit::Gram)).add(
            Quantity::new(Number::from_i64s(8, 3), Unit::base(BaseUnit::Gram).pow(Number::from_i64(2))),
        ).is_err()
    );
    assert!(
        Quantity::new(Number::from_i64s(-8, 3), Unit::base(BaseUnit::Second)).add(
            Quantity::unitless(Number::from_i64s(8, 3)),
        ).is_err()
    );
}

#[test]
fn sub() {
    assert_eq!(
        Quantity::new(Number::from_i64s(16, 3), Unit::base(BaseUnit::Gram)),
        Quantity::new(Number::from_i64s(8, 3), Unit::base(BaseUnit::Gram)).sub(
            Quantity::new(Number::from_i64s(-8, 3), Unit::base(BaseUnit::Gram)),
        ).expect("neg failed")
    );
    assert_eq!(
        Quantity::new(Number::zero(), Unit::base(BaseUnit::Second)),
        Quantity::new(Number::from_i64s(8, 3), Unit::base(BaseUnit::Second)).sub(
            Quantity::new(Number::from_i64s(8, 3), Unit::base(BaseUnit::Second)),
        ).expect("neg failed")
    );
}

#[test]
fn sub_error() {
    assert!(
        Quantity::new(Number::from_i64s(8, 3), Unit::base(BaseUnit::Gram)).sub(
            Quantity::new(Number::from_i64s(8, 3), Unit::base(BaseUnit::Second)),
        ).is_err()
    );
    assert!(
        Quantity::new(Number::from_i64s(8, 3), Unit::base(BaseUnit::Gram)).sub(
            Quantity::new(Number::from_i64s(8, 3), Unit::base(BaseUnit::Gram).pow(Number::from_i64(2))),
        ).is_err()
    );
    assert!(
        Quantity::new(Number::from_i64s(-8, 3), Unit::base(BaseUnit::Second)).sub(
            Quantity::unitless(Number::from_i64s(8, 3)),
        ).is_err()
    );
}

#[test]
fn mul() {
    assert_eq!(
        Quantity::new(Number::from_i64s(-64, 9), Unit::base(BaseUnit::Gram).pow(Number::from_i64(2))),
        Quantity::new(Number::from_i64s(8, 3), Unit::base(BaseUnit::Gram)).mul(
            Quantity::new(Number::from_i64s(-8, 3), Unit::base(BaseUnit::Gram)),
        ).expect("neg failed")
    );
    assert_eq!(
        Quantity::unitless(Number::from_i64s(-64, 9)),
        Quantity::unitless(Number::from_i64s(8, 3)).mul(
            Quantity::unitless(Number::from_i64s(-8, 3)),
        ).expect("neg failed")
    );
    assert_eq!(
        Quantity::new(Number::from_i64s(64, 9), Unit::base(BaseUnit::Second)),
        Quantity::new(Number::from_i64s(8, 3), Unit::base(BaseUnit::Second)).mul(
            Quantity::unitless(Number::from_i64s(8, 3)),
        ).expect("neg failed")
    );
    assert_eq!(
        Quantity::new(Number::from_i64s(16, 21), Unit::base(BaseUnit::Second).mul(Unit::base(BaseUnit::Gram))),
        Quantity::new(Number::from_i64s(8, 3), Unit::base(BaseUnit::Second)).mul(
            Quantity::new(Number::from_i64s(2, 7), Unit::base(BaseUnit::Gram)),
        ).expect("neg failed")
    );
}

#[test]
fn div() {
    assert_eq!(
        Quantity::unitless(Number::neg_one()),
        Quantity::new(Number::from_i64s(8, 3), Unit::base(BaseUnit::Gram)).div(
            Quantity::new(Number::from_i64s(-8, 3), Unit::base(BaseUnit::Gram)),
        ).expect("neg failed")
    );
    assert_eq!(
        Quantity::new(Number::one(), Unit::base(BaseUnit::Second)),
        Quantity::new(Number::from_i64s(8, 3), Unit::base(BaseUnit::Second)).div(
            Quantity::unitless(Number::from_i64s(8, 3)),
        ).expect("neg failed")
    );
    assert_eq!(
        Quantity::new(Number::from_i64s(56, 6), Unit::base(BaseUnit::Second).mul(Unit::base(BaseUnit::Gram).pow(Number::neg_one()))),
        Quantity::new(Number::from_i64s(8, 3), Unit::base(BaseUnit::Second)).div(
            Quantity::new(Number::from_i64s(2, 7), Unit::base(BaseUnit::Gram)),
        ).expect("neg failed")
    );
}

#[test]
fn pow() {
    assert_eq!(
        Quantity::new(Number::from_i64s(8, 3), Unit::base(BaseUnit::Gram)),
        Quantity::new(Number::from_i64s(8, 3), Unit::base(BaseUnit::Gram)).pow(
            Quantity::unitless(Number::one())
        ).expect("neg failed")
    );
    assert_eq!(
        Quantity::new(Number::from_i64s(64, 9), Unit::base(BaseUnit::Gram).pow(Number::from_i64(2))),
        Quantity::new(Number::from_i64s(8, 3), Unit::base(BaseUnit::Gram)).pow(
            Quantity::unitless(Number::from_i64(2))
        ).expect("neg failed")
    );
    assert_eq!(
        Quantity::new(Number::from_i64s(9, 64), Unit::base(BaseUnit::Gram).pow(Number::from_i64(-2))),
        Quantity::new(Number::from_i64s(8, 3), Unit::base(BaseUnit::Gram)).pow(
            Quantity::unitless(Number::from_i64(-2))
        ).expect("neg failed")
    );
}

#[test]
fn pow_error() {
    assert!(
        Quantity::unitless(Number::from_i64s(8, 3)).pow(
            Quantity::new(Number::one(), Unit::base(BaseUnit::Second))
        ).is_err()
    );
    assert!(
        Quantity::new(Number::zero(), Unit::base(BaseUnit::Gram)).pow(
            Quantity::unitless(Number::from_i64(0))
        ).is_err()
    );
    assert!(
        Quantity::new(Number::zero(), Unit::base(BaseUnit::Gram)).pow(
            Quantity::unitless(Number::from_i64(-2))
        ).is_err()
    );
}

