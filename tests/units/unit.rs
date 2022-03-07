
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

