
use std::{collections::HashMap, cmp::Ordering, ops::*};
use num::traits::Pow;

use crate::{Context, ContextFn, EvalError, Number};

use super::{Quantity, Unit, unit::BaseUnit};

pub struct QuantityContext {
    vars: HashMap<String, Quantity>,
    funcs: HashMap<String, ContextFn<Quantity>>,
    units: HashMap<String, Quantity>,
}

fn check_length(args: Vec<Quantity>, min: usize, max: usize) -> Result<Vec<Quantity>, EvalError> {
    if args.len() < min {
        return Err(EvalError::ArgumentMismatch("Too few arguments to function".to_owned()));
    } else if args.len() > max {
        return Err(EvalError::ArgumentMismatch("Too many arguments to function".to_owned()));
    } else {
        return Ok(args);
    }
}

fn min(mut args: Vec<Quantity>) -> Result<Quantity, EvalError> {
    args = check_length(args, 1, usize::MAX)?;
    let mut m = 0;
    for i in 1..args.len() {
        let ord = args[i].partial_cmp(&args[m]);
        if let Some(o) = ord {
            if o == Ordering::Less {
                m = i;
            }
        } else {
            return Err(EvalError::NotSupported("Values in min function are not comparable".to_owned()));
        }
    }
    return Ok(args[m].clone());
}

fn max(mut args: Vec<Quantity>) -> Result<Quantity, EvalError> {
    args = check_length(args, 1, usize::MAX)?;
    let mut m = 0;
    for i in 1..args.len() {
        let ord = args[i].partial_cmp(&args[m]);
        if let Some(o) = ord {
            if o == Ordering::Greater {
                m = i;
            }
        } else {
            return Err(EvalError::NotSupported("Values in max function are not comparable".to_owned()));
        }
    }
    return Ok(args[m].clone());
}

fn unitless_function<F: Fn(&Number) -> Number>(mut vec: Vec<Quantity>, f: F) -> Result<Quantity, EvalError> {
    vec = check_length(vec, 1, 1)?;
    if vec[0].is_unitless() {
        let num = f(vec[0].coefficient());
        return num.nan_to_err().map(|x| Quantity::unitless(x));
    } else {
        return Err(EvalError::NotSupported("Function can only be applied to unitless quantity".to_owned()));
    }
}

fn si_unit_prefix() -> Vec<(&'static str, Number)> {
    vec![
        ("Y", Number::from_i128(1_000_000_000_000_000_000_000_000)),
        ("Z", Number::from_i128(1_000_000_000_000_000_000_000)),
        ("E", Number::from_i128(1_000_000_000_000_000_000)),
        ("P", Number::from_i128(1_000_000_000_000_000)),
        ("T", Number::from_i128(1_000_000_000_000)),
        ("G", Number::from_i128(1_000_000_000)),
        ("M", Number::from_i128(1_000_000)),
        ("k", Number::from_i128(1_000)),
        ("h", Number::from_i128(100)),
        ("da", Number::from_i128(10)),
        ("", Number::from_i128(1)),
        ("d", Number::from_i128s(1, 10)),
        ("c", Number::from_i128s(1, 100)),
        ("m", Number::from_i128s(1, 1_000)),
        ("u", Number::from_i128s(1, 1_000_000)),
        ("n", Number::from_i128s(1, 1_000_000_000)),
        ("p", Number::from_i128s(1, 1_000_000_000_000)),
        ("f", Number::from_i128s(1, 1_000_000_000_000_000)),
        ("a", Number::from_i128s(1, 1_000_000_000_000_000_000)),
        ("z", Number::from_i128s(1, 1_000_000_000_000_000_000_000)),
        ("y", Number::from_i128s(1, 1_000_000_000_000_000_000_000_000)),
    ]
}

fn si_derived_units() -> Vec<(&'static str, Quantity)> {
    vec![
        ("rad", Quantity::unitless(Number::one())),
        ("sr", Quantity::unitless(Number::one())),
        ("Hz", Quantity::new(Number::one(), Unit::base(BaseUnit::Second).pow(Number::neg_one()))),
        ("N", Quantity::new(Number::from_i64(1000),
            Unit::base(BaseUnit::Gram)
                .mul(Unit::base(BaseUnit::Meter))
                .mul(Unit::base(BaseUnit::Second).pow(Number::from_i64(-2)))
        )),
        ("Pa", Quantity::new(Number::from_i64(1000),
            Unit::base(BaseUnit::Gram)
                .mul(Unit::base(BaseUnit::Meter).pow(Number::neg_one()))
                .mul(Unit::base(BaseUnit::Second).pow(Number::from_i64(-2)))
        )),
        ("J", Quantity::new(Number::from_i64(1000),
            Unit::base(BaseUnit::Gram)
                .mul(Unit::base(BaseUnit::Meter).pow(Number::from_i64(2)))
                .mul(Unit::base(BaseUnit::Second).pow(Number::from_i64(-2)))
        )),
        ("W", Quantity::new(Number::from_i64(1000),
            Unit::base(BaseUnit::Gram)
                .mul(Unit::base(BaseUnit::Meter).pow(Number::from_i64(2)))
                .mul(Unit::base(BaseUnit::Second).pow(Number::from_i64(-3)))
        )),
        ("C", Quantity::new(Number::one(), Unit::base(BaseUnit::Second).mul(Unit::base(BaseUnit::Ampere)))),
        ("V", Quantity::new(Number::from_i64(1000),
            Unit::base(BaseUnit::Gram)
                .mul(Unit::base(BaseUnit::Meter).pow(Number::from_i64(2)))
                .mul(Unit::base(BaseUnit::Second).pow(Number::from_i64(-3)))
                .mul(Unit::base(BaseUnit::Ampere).pow(Number::neg_one()))
        )),
        ("F", Quantity::new(Number::from_i64s(1, 1000),
            Unit::base(BaseUnit::Gram).pow(Number::neg_one())
                .mul(Unit::base(BaseUnit::Meter).pow(Number::from_i64(-2)))
                .mul(Unit::base(BaseUnit::Second).pow(Number::from_i64(4)))
                .mul(Unit::base(BaseUnit::Ampere).pow(Number::from_i64(2)))
        )),
        ("ohm", Quantity::new(Number::from_i64(1000),
            Unit::base(BaseUnit::Gram)
                .mul(Unit::base(BaseUnit::Meter).pow(Number::from_i64(2)))
                .mul(Unit::base(BaseUnit::Second).pow(Number::from_i64(-3)))
                .mul(Unit::base(BaseUnit::Ampere).pow(Number::from_i64(-2)))
        )),
        ("S", Quantity::new(Number::from_i64s(1, 1000),
            Unit::base(BaseUnit::Gram).pow(Number::neg_one())
                .mul(Unit::base(BaseUnit::Meter).pow(Number::from_i64(-2)))
                .mul(Unit::base(BaseUnit::Second).pow(Number::from_i64(3)))
                .mul(Unit::base(BaseUnit::Ampere).pow(Number::from_i64(2)))
        )),
        ("Wb", Quantity::new(Number::from_i64(1000),
            Unit::base(BaseUnit::Gram)
                .mul(Unit::base(BaseUnit::Meter).pow(Number::from_i64(2)))
                .mul(Unit::base(BaseUnit::Second).pow(Number::from_i64(-2)))
                .mul(Unit::base(BaseUnit::Ampere).pow(Number::neg_one()))
        )),
        ("T", Quantity::new(Number::from_i64(1000),
            Unit::base(BaseUnit::Gram)
                .mul(Unit::base(BaseUnit::Second).pow(Number::from_i64(-2)))
                .mul(Unit::base(BaseUnit::Ampere).pow(Number::neg_one()))
        )),
        ("H", Quantity::new(Number::from_i64(1000),
            Unit::base(BaseUnit::Gram)
                .mul(Unit::base(BaseUnit::Meter).pow(Number::from_i64(2)))
                .mul(Unit::base(BaseUnit::Second).pow(Number::from_i64(-2)))
                .mul(Unit::base(BaseUnit::Ampere).pow(Number::from_i64(-2)))
        )),
        ("lm", Quantity::new(Number::one(), Unit::base(BaseUnit::Candela))),
        ("lx", Quantity::new(Number::one(),
            Unit::base(BaseUnit::Candela).mul(Unit::base(BaseUnit::Meter).pow(Number::from_i64(-2)))
        )),
        ("Bq", Quantity::new(Number::one(), Unit::base(BaseUnit::Candela).pow(Number::neg_one()))),
        ("Gy", Quantity::new(Number::one(),
            Unit::base(BaseUnit::Gram).pow(Number::from_i64(2))
                .mul(Unit::base(BaseUnit::Second).pow(Number::from_i64(-2)))
        )),
        ("Sv", Quantity::new(Number::one(),
            Unit::base(BaseUnit::Gram).pow(Number::from_i64(2))
                .mul(Unit::base(BaseUnit::Second).pow(Number::from_i64(-2)))
        )),
        ("kat", Quantity::new(Number::one(),
            Unit::base(BaseUnit::Mole).mul(Unit::base(BaseUnit::Second).pow(Number::neg_one()))
        )),
    ]
}

fn non_si_units() -> Vec<(&'static str, Quantity)> {
    vec![
        ("min", Quantity::new(Number::from_i64(60), Unit::base(BaseUnit::Second))),
        ("h", Quantity::new(Number::from_i64(3600), Unit::base(BaseUnit::Second))),
        ("d", Quantity::new(Number::from_i64(86400), Unit::base(BaseUnit::Second))),
        ("au", Quantity::new(Number::from_i64(149_597_870_700), Unit::base(BaseUnit::Meter))),
        ("ha", Quantity::new(Number::from_i64(10_000), Unit::base(BaseUnit::Meter).pow(Number::from_i64(2)))),
        ("l", Quantity::new(Number::from_i64s(1, 1_000), Unit::base(BaseUnit::Meter).pow(Number::from_i64(3)))),
        ("L", Quantity::new(Number::from_i64s(1, 1_000), Unit::base(BaseUnit::Meter).pow(Number::from_i64(3)))),
        ("t", Quantity::new(Number::from_i64(1_000_000), Unit::base(BaseUnit::Gram))),
        ("Da", Quantity::new(Number::from_i128s(166_053_904_020, (10 as i128).pow(35)), Unit::base(BaseUnit::Gram))),
        ("eV", Quantity::new(Number::from_i128s(1_602_176_634, (10 as i128).pow(25)),
            Unit::base(BaseUnit::Gram)
                .mul(Unit::base(BaseUnit::Meter).pow(Number::from_i64(2)))
                .mul(Unit::base(BaseUnit::Second).pow(Number::from_i64(-2)))
        )),
    ] 
}

fn add_functions_to_context(cxt: &mut QuantityContext) {
    cxt.set_function("abs", Box::new(|v| check_length(v, 1, 1)?[0].abs().nan_to_err()));
    cxt.set_function("sign", Box::new(|v| check_length(v, 1, 1)?[0].sign().nan_to_err()));
    cxt.set_function("sqrt", Box::new(|v| check_length(v, 1, 1)?[0].sqrt().nan_to_err()));
    cxt.set_function("cbrt", Box::new(|v| check_length(v, 1, 1)?[0].cbrt().nan_to_err()));
    cxt.set_function("min", Box::new(min));
    cxt.set_function("max", Box::new(max));
    cxt.set_function("floor",Box::new(|v| unitless_function(v, Number::floor)));
    cxt.set_function("ceil",Box::new(|v| unitless_function(v, Number::ceil)));
    cxt.set_function("round", Box::new(|v| unitless_function(v, Number::round)));
    cxt.set_function("trunc", Box::new(|v| unitless_function(v, Number::trunc)));
    cxt.set_function("fract", Box::new(|v| unitless_function(v, Number::fract)));
    cxt.set_function("ln", Box::new(|v| unitless_function(v, Number::ln)));
    cxt.set_function("log", Box::new(|v| unitless_function(v, Number::log)));
    cxt.set_function("sin", Box::new(|v| unitless_function(v, Number::sin)));
    cxt.set_function("cos", Box::new(|v| unitless_function(v, Number::cos)));
    cxt.set_function("tan", Box::new(|v| unitless_function(v, Number::tan)));
    cxt.set_function("asin", Box::new(|v| unitless_function(v, Number::asin)));
    cxt.set_function("acos", Box::new(|v| unitless_function(v, Number::acos)));
    cxt.set_function("atan", Box::new(|v| unitless_function(v, Number::atan)));
    cxt.set_function("atan2", Box::new(|mut v| {
        v = check_length(v, 2, 2)?;
        if v[0].is_unitless() && v[1].is_unitless() {
            let num = v[0].coefficient().atan2(v[1].coefficient());
            return num.nan_to_err().map(|x| Quantity::unitless(x));
        } else {
            return Err(EvalError::NotSupported("Function can only be applied to unitless quantity".to_owned()));
        }
    }));
    cxt.set_function("sinh", Box::new(|v| unitless_function(v, Number::sinh)));
    cxt.set_function("cosh", Box::new(|v| unitless_function(v, Number::cosh)));
    cxt.set_function("tanh", Box::new(|v| unitless_function(v, Number::tanh)));
    cxt.set_function("asinh", Box::new(|v| unitless_function(v, Number::asinh)));
    cxt.set_function("acosh", Box::new(|v| unitless_function(v, Number::acosh)));
    cxt.set_function("atanh", Box::new(|v| unitless_function(v, Number::atanh)));
}

impl QuantityContext {
    pub fn new() -> QuantityContext {
        let mut res = QuantityContext {
            vars: HashMap::new(), funcs: HashMap::new(), units: HashMap::new()
        };
        res.set_variable("pi", Quantity::pi());
        res.set_variable("e", Quantity::e());
        add_functions_to_context(&mut res);
        for (symb, unit) in non_si_units() {
            res.units.insert(symb.to_owned(), unit);
        }
        for (pr_symbol, pr) in si_unit_prefix() {
            for (symb, unit) in si_derived_units() {
                let symbol = format!("{}{}", pr_symbol, symb);
                res.units.insert(symbol, Quantity::unitless(pr.clone()).mul(unit).unwrap());
            }
            for base in BaseUnit::all() {
                let symbol = format!("{}{}", pr_symbol, base.symbol());
                res.units.insert(symbol, Quantity::new(pr.clone(), Unit::base(base)));
            }
        }
        return res;
    }
}

impl Context<Quantity> for QuantityContext {
    fn set_variable(&mut self, name: &str, value: Quantity) {
        self.vars.insert(name.to_owned(), value);
    }

    fn set_function(&mut self, name: &str, value: ContextFn<Quantity>) {
        self.funcs.insert(name.to_owned(), value);
    }

    fn get_variable(&self, name: &str) -> Option<Quantity> {
        return self.vars.get(name)
            .or_else(|| self.units.get(name)
                .or_else(|| self.units.get(&name.to_lowercase())))
            .and_then(|n| Some(n.clone()))
    }

    fn get_function<'a>(&'a self, name: &str) -> Option<&'a ContextFn<Quantity>> {
        return self.funcs.get(name);
    }
}

