
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
        return Err(EvalError::UnitError("Function can only be applied to unitless quantity".to_owned()));
    }
}

fn si_unit_prefix() -> Vec<(Vec<&'static str>, Number)> {
    vec![
        (vec!["yotta", "Y"], Number::from_i128(1_000_000_000_000_000_000_000_000)),
        (vec!["zetta", "Z"], Number::from_i128(1_000_000_000_000_000_000_000)),
        (vec!["exa", "E"], Number::from_i128(1_000_000_000_000_000_000)),
        (vec!["peta", "P"], Number::from_i128(1_000_000_000_000_000)),
        (vec!["tera", "T"], Number::from_i128(1_000_000_000_000)),
        (vec!["giga", "G"], Number::from_i128(1_000_000_000)),
        (vec!["mega", "M"], Number::from_i128(1_000_000)),
        (vec!["kilo", "k"], Number::from_i128(1_000)),
        (vec!["hecto", "h"], Number::from_i128(100)),
        (vec!["deca", "da"], Number::from_i128(10)),
        (vec![""], Number::from_i128(1)),
        (vec!["deci", "d"], Number::from_i128s(1, 10)),
        (vec!["centi", "c"], Number::from_i128s(1, 100)),
        (vec!["milli", "m"], Number::from_i128s(1, 1_000)),
        (vec!["micro", "u"], Number::from_i128s(1, 1_000_000)),
        (vec!["nano", "n"], Number::from_i128s(1, 1_000_000_000)),
        (vec!["pico", "p"], Number::from_i128s(1, 1_000_000_000_000)),
        (vec!["femto", "f"], Number::from_i128s(1, 1_000_000_000_000_000)),
        (vec!["atto", "a"], Number::from_i128s(1, 1_000_000_000_000_000_000)),
        (vec!["zepto", "z"], Number::from_i128s(1, 1_000_000_000_000_000_000_000)),
        (vec!["yocto", "y"], Number::from_i128s(1, 1_000_000_000_000_000_000_000_000)),
    ]
}

fn si_units() -> Vec<(Vec<&'static str>, Quantity)> {
    vec![
        (vec!["second", "seconds", "s"], Quantity::new(Number::one(), Unit::base(BaseUnit::Second))),
        (vec!["meter", "meters", "metre", "m"], Quantity::new(Number::one(), Unit::base(BaseUnit::Meter))),
        (vec!["gram", "grams", "g"], Quantity::new(Number::one(), Unit::base(BaseUnit::Gram))),
        (vec!["ampere", "amperes", "A"], Quantity::new(Number::one(), Unit::base(BaseUnit::Ampere))),
        (vec!["mole", "moles", "mol"], Quantity::new(Number::one(), Unit::base(BaseUnit::Mole))),
        (vec!["kelvin", "K"], Quantity::new(Number::one(), Unit::base(BaseUnit::Kelvin))),
        (vec!["candela", "cd"], Quantity::new(Number::one(), Unit::base(BaseUnit::Candela))),
        (vec!["radian", "rad"], Quantity::unitless(Number::one())),
        (vec!["steradian", "sr"], Quantity::unitless(Number::one())),
        (vec!["hertz", "Hz"], Quantity::new(Number::one(), Unit::base(BaseUnit::Second).pow(Number::neg_one()))),
        (vec!["newton", "newtons", "N"], Quantity::new(Number::from_i64(1000),
            Unit::base(BaseUnit::Gram)
                .mul(Unit::base(BaseUnit::Meter))
                .mul(Unit::base(BaseUnit::Second).pow(Number::from_i64(-2)))
        )),
        (vec!["pascal", "Pa"], Quantity::new(Number::from_i64(1000),
            Unit::base(BaseUnit::Gram)
                .mul(Unit::base(BaseUnit::Meter).pow(Number::neg_one()))
                .mul(Unit::base(BaseUnit::Second).pow(Number::from_i64(-2)))
        )),
        (vec!["joule", "joules", "J"], Quantity::new(Number::from_i64(1000),
            Unit::base(BaseUnit::Gram)
                .mul(Unit::base(BaseUnit::Meter).pow(Number::from_i64(2)))
                .mul(Unit::base(BaseUnit::Second).pow(Number::from_i64(-2)))
        )),
        (vec!["watt", "watts", "W"], Quantity::new(Number::from_i64(1000),
            Unit::base(BaseUnit::Gram)
                .mul(Unit::base(BaseUnit::Meter).pow(Number::from_i64(2)))
                .mul(Unit::base(BaseUnit::Second).pow(Number::from_i64(-3)))
        )),
        (vec!["coulomb", "coulombs", "C"], Quantity::new(Number::one(), Unit::base(BaseUnit::Second).mul(Unit::base(BaseUnit::Ampere)))),
        (vec!["volt", "volts", "V"], Quantity::new(Number::from_i64(1000),
            Unit::base(BaseUnit::Gram)
                .mul(Unit::base(BaseUnit::Meter).pow(Number::from_i64(2)))
                .mul(Unit::base(BaseUnit::Second).pow(Number::from_i64(-3)))
                .mul(Unit::base(BaseUnit::Ampere).pow(Number::neg_one()))
        )),
        (vec!["farad", "F"], Quantity::new(Number::from_i64s(1, 1000),
            Unit::base(BaseUnit::Gram).pow(Number::neg_one())
                .mul(Unit::base(BaseUnit::Meter).pow(Number::from_i64(-2)))
                .mul(Unit::base(BaseUnit::Second).pow(Number::from_i64(4)))
                .mul(Unit::base(BaseUnit::Ampere).pow(Number::from_i64(2)))
        )),
        (vec!["ohms", "ohm"], Quantity::new(Number::from_i64(1000),
            Unit::base(BaseUnit::Gram)
                .mul(Unit::base(BaseUnit::Meter).pow(Number::from_i64(2)))
                .mul(Unit::base(BaseUnit::Second).pow(Number::from_i64(-3)))
                .mul(Unit::base(BaseUnit::Ampere).pow(Number::from_i64(-2)))
        )),
        (vec!["siemens", "S"], Quantity::new(Number::from_i64s(1, 1000),
            Unit::base(BaseUnit::Gram).pow(Number::neg_one())
                .mul(Unit::base(BaseUnit::Meter).pow(Number::from_i64(-2)))
                .mul(Unit::base(BaseUnit::Second).pow(Number::from_i64(3)))
                .mul(Unit::base(BaseUnit::Ampere).pow(Number::from_i64(2)))
        )),
        (vec!["weber", "Wb"], Quantity::new(Number::from_i64(1000),
            Unit::base(BaseUnit::Gram)
                .mul(Unit::base(BaseUnit::Meter).pow(Number::from_i64(2)))
                .mul(Unit::base(BaseUnit::Second).pow(Number::from_i64(-2)))
                .mul(Unit::base(BaseUnit::Ampere).pow(Number::neg_one()))
        )),
        (vec!["tesla", "T"], Quantity::new(Number::from_i64(1000),
            Unit::base(BaseUnit::Gram)
                .mul(Unit::base(BaseUnit::Second).pow(Number::from_i64(-2)))
                .mul(Unit::base(BaseUnit::Ampere).pow(Number::neg_one()))
        )),
        (vec!["henry", "H"], Quantity::new(Number::from_i64(1000),
            Unit::base(BaseUnit::Gram)
                .mul(Unit::base(BaseUnit::Meter).pow(Number::from_i64(2)))
                .mul(Unit::base(BaseUnit::Second).pow(Number::from_i64(-2)))
                .mul(Unit::base(BaseUnit::Ampere).pow(Number::from_i64(-2)))
        )),
        (vec!["lumen", "lm"], Quantity::new(Number::one(), Unit::base(BaseUnit::Candela))),
        (vec!["lux", "lx"], Quantity::new(Number::one(),
            Unit::base(BaseUnit::Candela).mul(Unit::base(BaseUnit::Meter).pow(Number::from_i64(-2)))
        )),
        (vec!["becquerel", "Bq"], Quantity::new(Number::one(), Unit::base(BaseUnit::Candela).pow(Number::neg_one()))),
        (vec!["gray", "Gy"], Quantity::new(Number::one(),
            Unit::base(BaseUnit::Gram).pow(Number::from_i64(2))
                .mul(Unit::base(BaseUnit::Second).pow(Number::from_i64(-2)))
        )),
        (vec!["sievert", "Sv"], Quantity::new(Number::one(),
            Unit::base(BaseUnit::Gram).pow(Number::from_i64(2))
                .mul(Unit::base(BaseUnit::Second).pow(Number::from_i64(-2)))
        )),
        (vec!["katal", "kat"], Quantity::new(Number::one(),
            Unit::base(BaseUnit::Mole).mul(Unit::base(BaseUnit::Second).pow(Number::neg_one()))
        )),
        // Not SI units, but can be used with SI prefix
        (vec!["litre", "liter", "l", "L"], Quantity::new(Number::from_i64s(1, 1_000), Unit::base(BaseUnit::Meter).pow(Number::from_i64(3)))),
        (vec!["tonne", "tonnes", "t"], Quantity::new(Number::from_i64(1_000_000), Unit::base(BaseUnit::Gram))),
        (vec!["electronvolt", "electronvolts", "eV"], Quantity::new(Number::from_i128s(1_602_176_634, (10 as i128).pow(25)),
            Unit::base(BaseUnit::Gram)
                .mul(Unit::base(BaseUnit::Meter).pow(Number::from_i64(2)))
                .mul(Unit::base(BaseUnit::Second).pow(Number::from_i64(-2)))
        )),
    ]
}

fn non_si_units() -> Vec<(Vec<&'static str>, Quantity)> {
    vec![
        // Imperial length
        (vec!["twips", "twip"], Quantity::new(Number::from_i128s(176_389, (10 as i128).pow(10)), Unit::base(BaseUnit::Meter))),
        (vec!["thou", "thous", "th"], Quantity::new(Number::from_i128s(254, (10 as i128).pow(7)), Unit::base(BaseUnit::Meter))),
        (vec!["barleycorn", "barleycorns", "Bc"], Quantity::new(Number::from_i128s(84_667, (10 as i128).pow(7)), Unit::base(BaseUnit::Meter))),
        (vec!["inch", "inches", "in"], Quantity::new(Number::from_i128s(0_0254, (10 as i128).pow(4)), Unit::base(BaseUnit::Meter))),
        (vec!["hands", "hand"], Quantity::new(Number::from_i128s(0_1016, (10 as i128).pow(4)), Unit::base(BaseUnit::Meter))),
        (vec!["foot", "feet", "ft"], Quantity::new(Number::from_i128s(0_3048, (10 as i128).pow(4)), Unit::base(BaseUnit::Meter))),
        (vec!["yard", "yards", "yd"], Quantity::new(Number::from_i128s(0_9144, (10 as i128).pow(4)), Unit::base(BaseUnit::Meter))),
        (vec!["chain", "chains", "ch"], Quantity::new(Number::from_i128s(20_1168, (10 as i128).pow(4)), Unit::base(BaseUnit::Meter))),
        (vec!["furlong", "furlongs", "fur"], Quantity::new(Number::from_i128s(201_168, (10 as i128).pow(3)), Unit::base(BaseUnit::Meter))),
        (vec!["mile", "miles", "mi"], Quantity::new(Number::from_i128s(1609_344, (10 as i128).pow(3)), Unit::base(BaseUnit::Meter))),
        (vec!["league", "leagues", "lea"], Quantity::new(Number::from_i128s(4828_032, (10 as i128).pow(3)), Unit::base(BaseUnit::Meter))),
        (vec!["fathom", "fathoms", "ftm"], Quantity::new(Number::from_i128s(1_852, (10 as i128).pow(3)), Unit::base(BaseUnit::Meter))),
        (vec!["cables", "cable"], Quantity::new(Number::from_i128s(185_2, 10), Unit::base(BaseUnit::Meter))),
        (vec!["nauticalmile", "nauticalmiles", "nmi"], Quantity::new(Number::from_i128(1852), Unit::base(BaseUnit::Meter))),
        (vec!["links", "link"], Quantity::new(Number::from_i128s(0_201168, (10 as i128).pow(6)), Unit::base(BaseUnit::Meter))),
        (vec!["rods", "rod"], Quantity::new(Number::from_i128s(5_0292, (10 as i128).pow(4)), Unit::base(BaseUnit::Meter))),
        // Imperial area
        (vec!["perches", "perch"], Quantity::new(
            Number::from_i128s(25_29285264, (10 as i128).pow(8)), Unit::base(BaseUnit::Meter).pow(Number::from_i64(2))
        )),
        (vec!["roods", "rood"], Quantity::new(
            Number::from_i128s(1011_7141056, (10 as i128).pow(7)), Unit::base(BaseUnit::Meter).pow(Number::from_i64(2))
        )),
        (vec!["acres", "acre"], Quantity::new(
            Number::from_i128s(4046_8564224, (10 as i128).pow(7)), Unit::base(BaseUnit::Meter).pow(Number::from_i64(2))
        )),
        (vec!["squaremile", "squaremiles", "sqmi"], Quantity::new(
            Number::from_i128s(2589988_110336, 1_000_000), Unit::base(BaseUnit::Meter).pow(Number::from_i64(2))
        )),
        // Imperial volume
        (vec!["fluidounce", "fluidounces", "floz"], Quantity::new(
            Number::from_i128s(28_4130625, (10 as i128).pow(13)), Unit::base(BaseUnit::Meter).pow(Number::from_i64(3))
        )),
        (vec!["gill", "gills", "gi"], Quantity::new(
            Number::from_i128s(142_0653125, (10 as i128).pow(13)), Unit::base(BaseUnit::Meter).pow(Number::from_i64(3))
        )),
        (vec!["pint", "pints", "pt"], Quantity::new(
            Number::from_i128s(568_26125, (10 as i128).pow(11)), Unit::base(BaseUnit::Meter).pow(Number::from_i64(3))
        )),
        (vec!["quart", "quarts", "qt"], Quantity::new(
            Number::from_i128s(1136_5225, (10 as i128).pow(10)), Unit::base(BaseUnit::Meter).pow(Number::from_i64(3))
        )),
        (vec!["gallon", "gallons", "gal"], Quantity::new(
            Number::from_i128s(4546_09, (10 as i128).pow(8)), Unit::base(BaseUnit::Meter).pow(Number::from_i64(3))
        )),
        // Imperial mass
        (vec!["grain", "grains", "gr"], Quantity::new(Number::from_i128s(0_06479891, (10 as i128).pow(8)), Unit::base(BaseUnit::Gram))),
        (vec!["drachm", "drachms", "dr"], Quantity::new(Number::from_i128s(1_7718451953125, (10 as i128).pow(13)), Unit::base(BaseUnit::Gram))),
        (vec!["ounce", "ounces", "oz"], Quantity::new(Number::from_i128s(28_349523125, (10 as i128).pow(9)), Unit::base(BaseUnit::Gram))),
        (vec!["pound", "pounds", "lb"], Quantity::new(Number::from_i128s(453_59237, (10 as i128).pow(5)), Unit::base(BaseUnit::Gram))),
        (vec!["stone", "stones", "st"], Quantity::new(Number::from_i128s(6350_29318, (10 as i128).pow(5)), Unit::base(BaseUnit::Gram))),
        (vec!["quarter", "quarters", "qr"], Quantity::new(Number::from_i128s(12700_58636, (10 as i128).pow(5)), Unit::base(BaseUnit::Gram))),
        (vec!["hundredweight", "hundredweights", "cwt"], Quantity::new(Number::from_i128s(50802_34544, (10 as i128).pow(5)), Unit::base(BaseUnit::Gram))),
        (vec!["tons", "ton"], Quantity::new(Number::from_i128s(1016046_9088, (10 as i128).pow(4)), Unit::base(BaseUnit::Gram))),
        (vec!["slugs", "slug"], Quantity::new(Number::from_i128s(14593_90294, (10 as i128).pow(5)), Unit::base(BaseUnit::Gram))),
        // SI accepted
        (vec!["minute", "minutes", "min"], Quantity::new(Number::from_i64(60), Unit::base(BaseUnit::Second))),
        (vec!["hour", "hours", "h"], Quantity::new(Number::from_i64(3600), Unit::base(BaseUnit::Second))),
        (vec!["day", "days", "d"], Quantity::new(Number::from_i64(86400), Unit::base(BaseUnit::Second))),
        (vec!["astronomicalunit", "au"], Quantity::new(Number::from_i64(149_597_870_700), Unit::base(BaseUnit::Meter))),
        (vec!["hectare", "hectares", "ha"], Quantity::new(Number::from_i64(10_000), Unit::base(BaseUnit::Meter).pow(Number::from_i64(2)))),
        (vec!["dalton", "Da"], Quantity::new(Number::from_i128s(166_053_904_020, (10 as i128).pow(35)), Unit::base(BaseUnit::Gram))),
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
            return Err(EvalError::UnitError("Function can only be applied to unitless quantity".to_owned()));
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
        for (symbs, unit) in non_si_units() {
            for symb in symbs {
                res.units.insert(symb.to_owned(), unit.clone());
            }
        }
        for (pr_symbols, pr) in si_unit_prefix() {
            for pr_symbol in pr_symbols {
                for (symbs, unit) in si_units() {
                    for symb in symbs {
                        let symbol = format!("{}{}", pr_symbol, symb);
                        res.units.insert(symbol, Quantity::unitless(pr.clone()).mul(unit.clone()).unwrap());
                    }
                }
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

