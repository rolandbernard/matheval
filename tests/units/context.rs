
use std::ops::*;
use num::traits::Pow;

use matheval::{Quantity, Number, QuantityContext, Context, Unit, BaseUnit};

#[test]
fn has_base_si_units() {
    let cnxt = QuantityContext::new();
    assert_eq!(
        Quantity::new(Number::one(), Unit::base(BaseUnit::Second)),
        cnxt.get_variable("s").expect("'s' is not in the context")
    );
    assert_eq!(
        Quantity::new(Number::one(), Unit::base(BaseUnit::Meter)),
        cnxt.get_variable("m").expect("'m' is not in the context")
    );
    assert_eq!(
        Quantity::new(Number::from_i64(1000), Unit::base(BaseUnit::Gram)),
        cnxt.get_variable("kg").expect("'kg' is not in the context")
    );
    assert_eq!(
        Quantity::new(Number::one(), Unit::base(BaseUnit::Kelvin)),
        cnxt.get_variable("K").expect("'K' is not in the context")
    );
    assert_eq!(
        Quantity::new(Number::one(), Unit::base(BaseUnit::Ampere)),
        cnxt.get_variable("A").expect("'A' is not in the context")
    );
    assert_eq!(
        Quantity::new(Number::one(), Unit::base(BaseUnit::Mole)),
        cnxt.get_variable("mol").expect("'mol' is not in the context")
    );
    assert_eq!(
        Quantity::new(Number::one(), Unit::base(BaseUnit::Candela)),
        cnxt.get_variable("cd").expect("'cd' is not in the context")
    );
}

#[test]
fn has_derived_si_units() {
    let cnxt = QuantityContext::new();
    assert_eq!(
        Quantity::new(Number::from_i64(1_000_000), Unit::base(BaseUnit::Gram)),
        cnxt.get_variable("t").expect("'t' is not in the context")
    );
    assert_eq!(
        Quantity::new(Number::from_i64s(1, 1000), Unit::base(BaseUnit::Meter).pow(Number::from_i64(3))),
        cnxt.get_variable("l").expect("'l' is not in the context")
    );
    assert_eq!(
        Quantity::new(Number::from_i64s(1, 1000), Unit::base(BaseUnit::Meter).pow(Number::from_i64(3))),
        cnxt.get_variable("L").expect("'L' is not in the context")
    );
    assert_eq!(
        Quantity::new(
            Number::from_i64(1000),
            Unit::base(BaseUnit::Meter).pow(Number::from_i64(2))
                .mul(Unit::base(BaseUnit::Second).pow(Number::from_i64(-3)))
                .mul(Unit::base(BaseUnit::Gram))
        ),
        cnxt.get_variable("W").expect("'W' is not in the context")
    );
}

#[test]
fn has_prefixed_units() {
    let cnxt = QuantityContext::new();
    assert_eq!(
        Quantity::new(Number::from_i64s(1, 1000), Unit::base(BaseUnit::Second)),
        cnxt.get_variable("ms").expect("'ms' is not in the context")
    );
    assert_eq!(
        Quantity::new(Number::from_i64(1_000_000_000_000), Unit::base(BaseUnit::Gram)),
        cnxt.get_variable("Mt").expect("'Mt' is not in the context")
    );
    assert_eq!(
        Quantity::new(Number::from_i64(1000), Unit::base(BaseUnit::Ampere)),
        cnxt.get_variable("kA").expect("'kA' is not in the context")
    );
    assert_eq!(
        Quantity::new(
            Number::from_i64s(1, 1_000_000_000),
            Unit::base(BaseUnit::Ampere).pow(Number::from_i64(2))
                .mul(Unit::base(BaseUnit::Second).pow(Number::from_i64(4)))
                .mul(Unit::base(BaseUnit::Meter).pow(Number::from_i64(-2)))
                .mul(Unit::base(BaseUnit::Gram).pow(Number::from_i64(-1)))
        ),
        cnxt.get_variable("uF").expect("'uF' is not in the context")
    );
}

#[test]
fn has_non_si_units() {
    let cnxt = QuantityContext::new();
    assert_eq!(
        Quantity::new(Number::from_i64(3600), Unit::base(BaseUnit::Second)),
        cnxt.get_variable("h").expect("'h' is not in the context")
    );
    assert_eq!(
        Quantity::new(Number::from_i64s(453_59237, 100000), Unit::base(BaseUnit::Gram)),
        cnxt.get_variable("lb").expect("'lb' is not in the context")
    );
    assert_eq!(
        Quantity::new(Number::from_i64s(0_3048, 10000), Unit::base(BaseUnit::Meter)),
        cnxt.get_variable("ft").expect("'ft' is not in the context")
    );
    assert_eq!(
        Quantity::new(Number::from_i64s(1609_344, 1000), Unit::base(BaseUnit::Meter)),
        cnxt.get_variable("mi").expect("'mi' is not in the context")
    );
    assert_eq!(
        Quantity::new(Number::from_i64s(4546_09, 100000000), Unit::base(BaseUnit::Meter).pow(Number::from_i64(3))),
        cnxt.get_variable("gal").expect("'gal' is not in the context")
    );
}

#[test]
fn has_named_units() {
    let cnxt = QuantityContext::new();
    assert_eq!(
        Quantity::new(Number::from_i64s(1, 1000), Unit::base(BaseUnit::Second)),
        cnxt.get_variable("milliseconds").expect("'milliseconds' is not in the context")
    );
    assert_eq!(
        Quantity::new(Number::from_i64(1_000_000_000_000), Unit::base(BaseUnit::Gram)),
        cnxt.get_variable("megatonne").expect("'megatonne' is not in the context")
    );
    assert_eq!(
        Quantity::new(Number::from_i64(1000), Unit::base(BaseUnit::Ampere)),
        cnxt.get_variable("kiloampere").expect("'kiloampere' is not in the context")
    );
    assert_eq!(
        Quantity::new(
            Number::from_i64s(1, 1_000_000_000),
            Unit::base(BaseUnit::Ampere).pow(Number::from_i64(2))
                .mul(Unit::base(BaseUnit::Second).pow(Number::from_i64(4)))
                .mul(Unit::base(BaseUnit::Meter).pow(Number::from_i64(-2)))
                .mul(Unit::base(BaseUnit::Gram).pow(Number::from_i64(-1)))
        ),
        cnxt.get_variable("microfarad").expect("'microfarad' is not in the context")
    );
    assert_eq!(
        Quantity::new(Number::from_i64(1_000_000), Unit::base(BaseUnit::Gram)),
        cnxt.get_variable("tonnes").expect("'tonnes' is not in the context")
    );
    assert_eq!(
        Quantity::new(Number::from_i64s(1, 1000), Unit::base(BaseUnit::Meter).pow(Number::from_i64(3))),
        cnxt.get_variable("liter").expect("'liter' is not in the context")
    );
    assert_eq!(
        Quantity::new(Number::from_i64s(1, 1000), Unit::base(BaseUnit::Meter).pow(Number::from_i64(3))),
        cnxt.get_variable("litre").expect("'litre' is not in the context")
    );
    assert_eq!(
        Quantity::new(
            Number::from_i64(1000),
            Unit::base(BaseUnit::Meter).pow(Number::from_i64(2))
                .mul(Unit::base(BaseUnit::Second).pow(Number::from_i64(-3)))
                .mul(Unit::base(BaseUnit::Gram))
        ),
        cnxt.get_variable("watt").expect("'watt' is not in the context")
    );
    assert_eq!(
        Quantity::new(Number::one(), Unit::base(BaseUnit::Second)),
        cnxt.get_variable("second").expect("'second' is not in the context")
    );
    assert_eq!(
        Quantity::new(Number::one(), Unit::base(BaseUnit::Meter)),
        cnxt.get_variable("metre").expect("'metre' is not in the context")
    );
    assert_eq!(
        Quantity::new(Number::from_i64(1000), Unit::base(BaseUnit::Gram)),
        cnxt.get_variable("kilogram").expect("'kilogram' is not in the context")
    );
    assert_eq!(
        Quantity::new(Number::one(), Unit::base(BaseUnit::Kelvin)),
        cnxt.get_variable("kelvin").expect("'kelvin' is not in the context")
    );
    assert_eq!(
        Quantity::new(Number::one(), Unit::base(BaseUnit::Ampere)),
        cnxt.get_variable("ampere").expect("'ampere' is not in the context")
    );
    assert_eq!(
        Quantity::new(Number::one(), Unit::base(BaseUnit::Mole)),
        cnxt.get_variable("mole").expect("'mole' is not in the context")
    );
    assert_eq!(
        Quantity::new(Number::one(), Unit::base(BaseUnit::Candela)),
        cnxt.get_variable("candela").expect("'candela' is not in the context")
    );
    assert_eq!(
        Quantity::new(Number::from_i64(3600), Unit::base(BaseUnit::Second)),
        cnxt.get_variable("hours").expect("'hours' is not in the context")
    );
    assert_eq!(
        Quantity::new(Number::from_i64s(453_59237, 100000), Unit::base(BaseUnit::Gram)),
        cnxt.get_variable("pound").expect("'pound' is not in the context")
    );
    assert_eq!(
        Quantity::new(Number::from_i64s(0_3048, 10000), Unit::base(BaseUnit::Meter)),
        cnxt.get_variable("feet").expect("'feet' is not in the context")
    );
    assert_eq!(
        Quantity::new(Number::from_i64s(1609_344, 1000), Unit::base(BaseUnit::Meter)),
        cnxt.get_variable("miles").expect("'miles' is not in the context")
    );
    assert_eq!(
        Quantity::new(Number::from_i64s(4546_09, 100000000), Unit::base(BaseUnit::Meter).pow(Number::from_i64(3))),
        cnxt.get_variable("gallons").expect("'gallons' is not in the context")
    );
}

#[test]
fn set_variable() {
    let mut cnxt = QuantityContext::new();
    let num = Quantity::unitless(Number::from_i64(12));
    cnxt.set_variable("a1", num.clone());
    let val = cnxt.get_variable("a1").expect("Context doesn't contain a1");
    assert_eq!(num, val);
}

#[test]
fn overwrite_variable() {
    let mut cnxt = QuantityContext::new();
    let num = Quantity::unitless(Number::from_i64(12));
    cnxt.set_variable("a1", num.clone());
    let num = Quantity::unitless(Number::from_i64(42));
    cnxt.set_variable("a1", num.clone());
    let val = cnxt.get_variable("a1").expect("Context doesn't contain a1");
    assert_eq!(num, val);
}

#[test]
fn overwrite_buildin_variable() {
    let mut cnxt = QuantityContext::new();
    let num = Quantity::unitless(Number::from_i64s(314, 100));
    cnxt.set_variable("pi", num.clone());
    let val = cnxt.get_variable("pi").expect("Context doesn't contain pi");
    assert_eq!(num, val);
}

#[test]
fn set_function() {
    let mut cnxt = QuantityContext::new();
    let num = Quantity::unitless(Number::from_i64(42));
    let func_num = num.clone();
    let func = move |_| Ok(func_num.clone());
    cnxt.set_function("func1", Box::new(func));
    let val = cnxt.get_function("func1")
        .expect("Context doesn't contain func1");
    assert_eq!(num, val(Vec::new()).expect("Function call failed"));
}

#[test]
fn overwrite_function() {
    let mut cnxt = QuantityContext::new();
    let num = Quantity::unitless(Number::from_i64(12));
    let func_num = num.clone();
    let func = move |_| Ok(func_num.clone());
    cnxt.set_function("func1", Box::new(func));
    let num = Quantity::unitless(Number::from_i64(42));
    let func_num = num.clone();
    let func = move |_| Ok(func_num.clone());
    cnxt.set_function("func1", Box::new(func));
    let val = cnxt.get_function("func1")
        .expect("Context doesn't contain func1");
    assert_eq!(num, val(Vec::new()).expect("Function call failed"));
}

#[test]
fn overwrite_buildin_function() {
    let mut cnxt = QuantityContext::new();
    let num = Quantity::unitless(Number::from_i64(42));
    let func_num = num.clone();
    let func = move |_| Ok(func_num.clone());
    cnxt.set_function("sin", Box::new(func));
    let val = cnxt.get_function("sin")
        .expect("Context doesn't contain sin");
    assert_eq!(num, val(Vec::new()).expect("Function call failed"));
}

#[test]
fn unknown_variable() {
    let cnxt = QuantityContext::new();
    assert_eq!(None, cnxt.get_variable("some_unknown_variable"));
}

#[test]
fn unknown_function() {
    let cnxt = QuantityContext::new();
    assert!(cnxt.get_function("some_unknown_function").is_none());
}

