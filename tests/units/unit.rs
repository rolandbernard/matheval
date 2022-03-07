
use std::ops::*;
use num::traits::Pow;

use matheval::{Unit, BaseUnit, Number};

#[test]
fn base_unit_symbols() {
    assert_eq!("s", BaseUnit::Second.symbol());
    assert_eq!("m", BaseUnit::Meter.symbol());
    assert_eq!("g", BaseUnit::Gram.symbol());
    assert_eq!("K", BaseUnit::Kelvin.symbol());
    assert_eq!("mol", BaseUnit::Mole.symbol());
    assert_eq!("cd", BaseUnit::Candela.symbol());
    assert_eq!("A", BaseUnit::Ampere.symbol());
}

#[test]
fn base_unit_try_from() {
    assert!(BaseUnit::try_from(0).is_ok());
    assert!(BaseUnit::try_from(1).is_ok());
    assert!(BaseUnit::try_from(2).is_ok());
    assert!(BaseUnit::try_from(3).is_ok());
    assert!(BaseUnit::try_from(4).is_ok());
    assert!(BaseUnit::try_from(5).is_ok());
    assert!(BaseUnit::try_from(6).is_ok());
}

#[test]
fn is_empty() {
    assert!(Unit::empty().is_empty());
    assert!(!Unit::base(BaseUnit::Second).is_empty());
    assert!(!Unit::base(BaseUnit::Meter).is_empty());
    assert!(!Unit::base(BaseUnit::Gram).is_empty());
    assert!(!Unit::base(BaseUnit::Candela).is_empty());
}

#[test]
fn equal() {
    assert_eq!(Unit::empty(), Unit::empty());
    assert_eq!(Unit::base(BaseUnit::Second), Unit::base(BaseUnit::Second));
    assert_eq!(Unit::base(BaseUnit::Meter), Unit::base(BaseUnit::Meter));
    assert_eq!(Unit::base(BaseUnit::Gram), Unit::base(BaseUnit::Gram));
    assert_eq!(Unit::base(BaseUnit::Kelvin), Unit::base(BaseUnit::Kelvin));
    assert_eq!(Unit::base(BaseUnit::Mole), Unit::base(BaseUnit::Mole));
    assert_eq!(Unit::base(BaseUnit::Candela), Unit::base(BaseUnit::Candela));
    assert_eq!(Unit::base(BaseUnit::Ampere), Unit::base(BaseUnit::Ampere));
    assert_eq!(
        Unit::base(BaseUnit::Ampere).pow(Number::from_i64(10)),
        Unit::base(BaseUnit::Ampere).pow(Number::from_i64(10))
    );
}

#[test]
fn not_equal() {
    assert_ne!(Unit::empty(), Unit::base(BaseUnit::Second));
    assert_ne!(Unit::empty(), Unit::base(BaseUnit::Meter));
    assert_ne!(Unit::empty(), Unit::base(BaseUnit::Gram));
    assert_ne!(Unit::empty(), Unit::base(BaseUnit::Kelvin));
    assert_ne!(Unit::empty(), Unit::base(BaseUnit::Mole));
    assert_ne!(Unit::empty(), Unit::base(BaseUnit::Candela));
    assert_ne!(Unit::empty(), Unit::base(BaseUnit::Ampere));
    assert_ne!(Unit::base(BaseUnit::Meter), Unit::base(BaseUnit::Second));
    assert_eq!(Unit::base(BaseUnit::Ampere).pow(Number::from_i64(1)), Unit::base(BaseUnit::Ampere));
}

#[test]
fn simple_mul() {
    assert_eq!(
        Unit::base(BaseUnit::Candela).pow(Number::from_i64(2)),
        Unit::base(BaseUnit::Candela).mul(Unit::base(BaseUnit::Candela))
    );
    assert_eq!(Unit::empty(), Unit::base(BaseUnit::Second).mul(Unit::base(BaseUnit::Second).pow(Number::neg_one())));
    assert_eq!(
        Unit::base(BaseUnit::Candela).mul(Unit::base(BaseUnit::Second)),
        Unit::base(BaseUnit::Second).mul(Unit::base(BaseUnit::Candela))
    );
}

#[test]
fn simple_div() {
    assert_eq!(Unit::empty(), Unit::base(BaseUnit::Candela).div(Unit::base(BaseUnit::Candela)));
    assert_eq!(
        Unit::base(BaseUnit::Second).pow(Number::from_i64(2)),
        Unit::base(BaseUnit::Second).div(Unit::base(BaseUnit::Second).pow(Number::neg_one()))
    );
    assert_eq!(
        Unit::base(BaseUnit::Second).pow(Number::from_i64(-2)),
        Unit::base(BaseUnit::Second).pow(Number::neg_one()).div(Unit::base(BaseUnit::Second))
    );
    assert_eq!(
        Unit::base(BaseUnit::Candela).pow(Number::neg_one()).div(Unit::base(BaseUnit::Second).pow(Number::neg_one())),
        Unit::base(BaseUnit::Second).div(Unit::base(BaseUnit::Candela))
    );
}

#[test]
fn simple_pow() {
    assert_eq!(
        Unit::base(BaseUnit::Second).pow(Number::from_i64(2)),
        Unit::base(BaseUnit::Second).mul(Unit::base(BaseUnit::Second))
    );
    assert_eq!(
        Unit::base(BaseUnit::Second).pow(Number::from_i64(5)),
        Unit::base(BaseUnit::Second)
            .mul(Unit::base(BaseUnit::Second)).mul(Unit::base(BaseUnit::Second))
            .mul(Unit::base(BaseUnit::Second)).mul(Unit::base(BaseUnit::Second))
    );
    assert_eq!(
        Unit::base(BaseUnit::Second).pow(Number::from_i64(-5)),
        Unit::empty().div(Unit::base(BaseUnit::Second))
            .div(Unit::base(BaseUnit::Second)).div(Unit::base(BaseUnit::Second))
            .div(Unit::base(BaseUnit::Second)).div(Unit::base(BaseUnit::Second))
    );
    assert_eq!(Unit::empty(), Unit::base(BaseUnit::Second).pow(Number::zero()));
}

#[test]
fn to_string_empty() {
    assert_eq!("", Unit::empty().to_string());
}

#[test]
fn to_string_base() {
    assert_eq!("s", Unit::base(BaseUnit::Second).to_string());
    assert_eq!("m", Unit::base(BaseUnit::Meter).to_string());
    assert_eq!("g", Unit::base(BaseUnit::Gram).to_string());
    assert_eq!("K", Unit::base(BaseUnit::Kelvin).to_string());
    assert_eq!("mol", Unit::base(BaseUnit::Mole).to_string());
    assert_eq!("cd", Unit::base(BaseUnit::Candela).to_string());
    assert_eq!("A", Unit::base(BaseUnit::Ampere).to_string());
}

#[test]
fn to_string_positive_int_power() {
    assert_eq!("s^2", Unit::base(BaseUnit::Second).pow(Number::from_i64(2)).to_string());
    assert_eq!("A^100", Unit::base(BaseUnit::Ampere).pow(Number::from_i64(100)).to_string());
    assert_eq!("mol^10000", Unit::base(BaseUnit::Mole).pow(Number::from_i64(10000)).to_string());
}

#[test]
fn to_string_negative_int_power() {
    assert_eq!("s^-2", Unit::base(BaseUnit::Second).pow(Number::from_i64(-2)).to_string());
    assert_eq!("A^-100", Unit::base(BaseUnit::Ampere).pow(Number::from_i64(-100)).to_string());
    assert_eq!("mol^-10000", Unit::base(BaseUnit::Mole).pow(Number::from_i64(-10000)).to_string());
}

#[test]
fn to_string_positive_fract_power() {
    assert_eq!("s^(2/7)", Unit::base(BaseUnit::Second).pow(Number::from_i64s(2, 7)).to_string());
    assert_eq!("A^(100/7)", Unit::base(BaseUnit::Ampere).pow(Number::from_i64s(100, 7)).to_string());
    assert_eq!("mol^(10000/7)", Unit::base(BaseUnit::Mole).pow(Number::from_i64s(10000, 7)).to_string());
}

#[test]
fn to_string_negative_fract_power() {
    assert_eq!("s^(-2/7)", Unit::base(BaseUnit::Second).pow(Number::from_i64s(-2, 7)).to_string());
    assert_eq!("A^(-100/7)", Unit::base(BaseUnit::Ampere).pow(Number::from_i64s(-100, 7)).to_string());
    assert_eq!("mol^(-10000/7)", Unit::base(BaseUnit::Mole).pow(Number::from_i64s(-10000, 7)).to_string());
}

#[test]
fn to_string_positive_float_power() {
    assert_eq!("s^2.75", Unit::base(BaseUnit::Second).pow(Number::Float(2.75)).to_string());
    assert_eq!("A^100.5", Unit::base(BaseUnit::Ampere).pow(Number::Float(100.5)).to_string());
    assert_eq!("mol^10000.125", Unit::base(BaseUnit::Mole).pow(Number::Float(10000.125)).to_string());
}

#[test]
fn to_string_negative_float_power() {
    assert_eq!("s^-2.75", Unit::base(BaseUnit::Second).pow(Number::Float(-2.75)).to_string());
    assert_eq!("A^-100.5", Unit::base(BaseUnit::Ampere).pow(Number::Float(-100.5)).to_string());
    assert_eq!("mol^-10000.125", Unit::base(BaseUnit::Mole).pow(Number::Float(-10000.125)).to_string());
}

#[test]
fn to_string_combined() {
    assert_eq!("m s", Unit::base(BaseUnit::Second).mul(Unit::base(BaseUnit::Meter)).to_string());
    assert_eq!("g m s", Unit::base(BaseUnit::Gram).mul(Unit::base(BaseUnit::Second)).mul(Unit::base(BaseUnit::Meter)).to_string());
    assert_eq!("g^-1 m^(3/4) s^12 mol^-1",
        Unit::base(BaseUnit::Gram).pow(Number::neg_one())
            .mul(Unit::base(BaseUnit::Second).pow(Number::from_i64(12)))
            .mul(Unit::base(BaseUnit::Meter).pow(Number::from_i64s(3, 4)))
            .div(Unit::base(BaseUnit::Mole)).to_string()
    );
}

