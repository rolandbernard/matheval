
use std::ops::*;
use num::traits::Pow;

use matheval::{Quantity, Number, Unit, BaseUnit, Expr};

#[test]
fn simple_integer_literal() {
    let parsed = Expr::parse("98765432109876543210")
        .expect("Failed parsing simple integer literal");
    assert_eq!(
        Quantity::unitless(Number::from_i128(98765432109876543210)),
        parsed.eval().expect("Evaluation failed")
    );
}

#[test]
fn binary_integer_literal() {
    let parsed = Expr::parse("0b1001")
        .expect("Failed parsing binary integer literal");
    assert_eq!(
        Quantity::unitless(Number::from_i64(9)),
        parsed.eval().expect("Evaluation failed")
    );
}

#[test]
fn octal_integer_literal() {
    let parsed = Expr::parse("0o12017")
        .expect("Failed parsing octal integer literal");
    assert_eq!(
        Quantity::unitless(Number::from_i64(5135)),
        parsed.eval().expect("Evaluation failed")
    );
}

#[test]
fn hex_integer_literal() {
    let parsed = Expr::parse("0xfa0c")
        .expect("Failed parsing hex integer literal");
    assert_eq!(
        Quantity::unitless(Number::from_i64(64012)),
        parsed.eval().expect("Evaluation failed")
    );
}

#[test]
fn nonint_literal() {
    let parsed = Expr::parse("12.56")
        .expect("Failed parsing non-integer literal");
    assert_eq!(
        Quantity::unitless(Number::from_i64s(1256, 100)),
        parsed.eval().expect("Evaluation failed")
    );
}

#[test]
fn exp_literal() {
    let parsed = Expr::parse("12.56e-10")
        .expect("Failed parsing exp literal");
    assert_eq!(
        Quantity::unitless(Number::from_i64s(1256, 1000000000000)),
        parsed.eval().expect("Evaluation failed")
    );
    let parsed = Expr::parse("12.56e+10")
        .expect("Failed parsing exp literal");
    assert_eq!(
        Quantity::unitless(Number::from_i64(125600000000)),
        parsed.eval().expect("Evaluation failed")
    );
}

#[test]
fn base_unit() {
    let parsed = Expr::parse("5m")
        .expect("Failed parsing simple add");
    assert_eq!(
        Quantity::new(Number::from_i64(5), Unit::base(BaseUnit::Meter)),
        parsed.eval().expect("Evaluation failed")
    );
}

#[test]
fn derived_unit() {
    let parsed = Expr::parse("5.5W")
        .expect("Failed parsing simple add");
    assert_eq!(
        Quantity::new(
            Number::from_i64(5500),
            Unit::base(BaseUnit::Meter).pow(Number::from_i64(2))
                .mul(Unit::base(BaseUnit::Second).pow(Number::from_i64(-3)))
                .mul(Unit::base(BaseUnit::Gram))
        ),
        parsed.eval().expect("Evaluation failed")
    );
}

#[test]
fn prefixed_unit() {
    let parsed = Expr::parse("5.5mW")
        .expect("Failed parsing simple add");
    assert_eq!(
        Quantity::new(
            Number::from_i64s(55, 10),
            Unit::base(BaseUnit::Meter).pow(Number::from_i64(2))
                .mul(Unit::base(BaseUnit::Second).pow(Number::from_i64(-3)))
                .mul(Unit::base(BaseUnit::Gram))
        ),
        parsed.eval().expect("Evaluation failed")
    );
}

#[test]
fn simple_add() {
    let parsed = Expr::parse("5cm + 6e-7m + 42.5cm")
        .expect("Failed parsing simple add");
    assert_eq!(
        Quantity::new(Number::from_i64s(47_50006, 10000000), Unit::base(BaseUnit::Meter)),
        parsed.eval().expect("Evaluation failed")
    );
}

#[test]
fn simple_sub() {
    let parsed = Expr::parse("5cm - 6e-7m - 42.5cm")
        .expect("Failed parsing simple add");
    assert_eq!(
        Quantity::new(Number::from_i64s(-0_3750006, 10000000), Unit::base(BaseUnit::Meter)),
        parsed.eval().expect("Evaluation failed")
    );
}

#[test]
fn mixed_add_sub() {
    let parsed = Expr::parse("5m + 6e-8km - 42.5e9nm + 15.7m")
        .expect("Failed parsing mixed add/sub");
    assert_eq!(
        Quantity::new(Number::from_i64s(-21_79994, 100000), Unit::base(BaseUnit::Meter)),
        parsed.eval().expect("Evaluation failed")
    );
}

#[test]
fn simple_mul() {
    let parsed = Expr::parse("5kg * 6e-5m * 42.5m")
        .expect("Failed parsing simple mul");
    assert_eq!(
        Quantity::new(
            Number::from_i64s(0_01275, 100),
            Unit::base(BaseUnit::Gram)
                .mul(Unit::base(BaseUnit::Meter).pow(Number::from_i64(2)))
        ),
        parsed.eval().expect("Evaluation failed")
    );
}

#[test]
fn simple_div() {
    let parsed = Expr::parse("(5e2m) / (6s) / (42.5s)")
        .expect("Failed parsing simple div");
    assert_eq!("100/51 m s^-2", parsed.eval::<Quantity>().expect("Evaluation failed").to_string());
}

#[test]
fn mixed_mul_div() {
    let parsed = Expr::parse("(5m) / (6e-5s) + (42.5e3mm) / (15e3ms)")
        .expect("Failed parsing mixed mul/div");
    assert_eq!("500017/6 m s^-1", parsed.eval::<Quantity>().expect("Evaluation failed").to_string());
}

#[test]
fn simple_pow() {
    let parsed = Expr::parse("(2m) ^ 3 ^ 2")
        .expect("Failed parsing simple pow");
    assert_eq!("512 m^9", parsed.eval::<Quantity>().expect("Evaluation failed").to_string());
    let parsed = Expr::parse("1.5 ^ 3 ^ 2")
        .expect("Failed parsing simple pow");
    assert_eq!("19683/512", parsed.eval::<Quantity>().expect("Evaluation failed").to_string());
    let parsed = Expr::parse("1.5 ^ 3.2 ^ 2.6")
        .expect("Failed parsing simple pow");
    assert_eq!("4202.383025252178", parsed.eval::<Quantity>().expect("Evaluation failed").to_string());
}

#[test]
fn mixed_mul_div_pow() {
    let parsed = Expr::parse("5 / (6Hz)^4 + (41s^2)^2 / 2^13")
        .expect("Failed parsing mixed mul/div/pow");
    assert_eq!("138721/663552 s^4", parsed.eval::<Quantity>().expect("Evaluation failed").to_string());
}

#[test]
fn mixed_parens() {
    let parsed = Expr::parse("(5 / 6)^(4 + 4)1^(2 / 2)^13")
        .expect("Failed parsing mixed parens");
    assert_eq!("390625/1679616", parsed.eval::<Quantity>().expect("Evaluation failed").to_string());
}

