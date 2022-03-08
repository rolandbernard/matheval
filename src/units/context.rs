
use std::{collections::HashMap, cmp::Ordering, ops::*};
use num::traits::Pow;

use crate::{Context, ContextFn, EvalError, Number};

use super::{Quantity, Unit, unit::BaseUnit};

pub struct QuantityContext {
    vars: HashMap<String, Quantity>,
    funcs: HashMap<String, Box<ContextFn<Quantity>>>,
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

fn get_base_unit_for(name: &str) -> Option<Quantity> {
    match name {
        "second" | "seconds" | "s" => Some(Quantity::new(Number::one(), Unit::base(BaseUnit::Second))),
        "meter" | "meters" | "metre" | "m" => Some(Quantity::new(Number::one(), Unit::base(BaseUnit::Meter))),
        "gram" | "grams" | "g" => Some(Quantity::new(Number::one(), Unit::base(BaseUnit::Gram))),
        "ampere" | "amperes" | "A" => Some(Quantity::new(Number::one(), Unit::base(BaseUnit::Ampere))),
        "mole" | "moles" | "mol" => Some(Quantity::new(Number::one(), Unit::base(BaseUnit::Mole))),
        "kelvin" | "K" => Some(Quantity::new(Number::one(), Unit::base(BaseUnit::Kelvin))),
        "candela" | "cd" => Some(Quantity::new(Number::one(), Unit::base(BaseUnit::Candela))),
        "radian" | "radians" | "rad" => Some(Quantity::unitless(Number::one())),
        "steradian" | "steradians" | "sr" => Some(Quantity::unitless(Number::one())),
        "hertz" | "Hz" => Some(Quantity::new(Number::one(), Unit::base(BaseUnit::Second).pow(Number::neg_one()))),
        "newton" | "newtons" | "N" => Some(Quantity::new(Number::from_i64(1000),
            Unit::base(BaseUnit::Gram)
                .mul(Unit::base(BaseUnit::Meter))
                .mul(Unit::base(BaseUnit::Second).pow(Number::from_i64(-2)))
        )),
        "pascal" | "Pa" => Some(Quantity::new(Number::from_i64(1000),
            Unit::base(BaseUnit::Gram)
                .mul(Unit::base(BaseUnit::Meter).pow(Number::neg_one()))
                .mul(Unit::base(BaseUnit::Second).pow(Number::from_i64(-2)))
        )),
        "joule" | "joules" | "J" => Some(Quantity::new(Number::from_i64(1000),
            Unit::base(BaseUnit::Gram)
                .mul(Unit::base(BaseUnit::Meter).pow(Number::from_i64(2)))
                .mul(Unit::base(BaseUnit::Second).pow(Number::from_i64(-2)))
        )),
        "watt" | "watts" | "W" => Some(Quantity::new(Number::from_i64(1000),
            Unit::base(BaseUnit::Gram)
                .mul(Unit::base(BaseUnit::Meter).pow(Number::from_i64(2)))
                .mul(Unit::base(BaseUnit::Second).pow(Number::from_i64(-3)))
        )),
        "coulomb" | "coulombs" | "C" => Some(Quantity::new(Number::one(), Unit::base(BaseUnit::Second).mul(Unit::base(BaseUnit::Ampere)))),
        "volt" | "volts" | "V" => Some(Quantity::new(Number::from_i64(1000),
            Unit::base(BaseUnit::Gram)
                .mul(Unit::base(BaseUnit::Meter).pow(Number::from_i64(2)))
                .mul(Unit::base(BaseUnit::Second).pow(Number::from_i64(-3)))
                .mul(Unit::base(BaseUnit::Ampere).pow(Number::neg_one()))
        )),
        "farad" | "farads" | "F" => Some(Quantity::new(Number::from_i64s(1, 1000),
            Unit::base(BaseUnit::Gram).pow(Number::neg_one())
                .mul(Unit::base(BaseUnit::Meter).pow(Number::from_i64(-2)))
                .mul(Unit::base(BaseUnit::Second).pow(Number::from_i64(4)))
                .mul(Unit::base(BaseUnit::Ampere).pow(Number::from_i64(2)))
        )),
        "ohm" | "ohms" => Some(Quantity::new(Number::from_i64(1000),
            Unit::base(BaseUnit::Gram)
                .mul(Unit::base(BaseUnit::Meter).pow(Number::from_i64(2)))
                .mul(Unit::base(BaseUnit::Second).pow(Number::from_i64(-3)))
                .mul(Unit::base(BaseUnit::Ampere).pow(Number::from_i64(-2)))
        )),
        "siemens" | "S" => Some(Quantity::new(Number::from_i64s(1, 1000),
            Unit::base(BaseUnit::Gram).pow(Number::neg_one())
                .mul(Unit::base(BaseUnit::Meter).pow(Number::from_i64(-2)))
                .mul(Unit::base(BaseUnit::Second).pow(Number::from_i64(3)))
                .mul(Unit::base(BaseUnit::Ampere).pow(Number::from_i64(2)))
        )),
        "weber" | "Wb" => Some(Quantity::new(Number::from_i64(1000),
            Unit::base(BaseUnit::Gram)
                .mul(Unit::base(BaseUnit::Meter).pow(Number::from_i64(2)))
                .mul(Unit::base(BaseUnit::Second).pow(Number::from_i64(-2)))
                .mul(Unit::base(BaseUnit::Ampere).pow(Number::neg_one()))
        )),
        "tesla" | "T" => Some(Quantity::new(Number::from_i64(1000),
            Unit::base(BaseUnit::Gram)
                .mul(Unit::base(BaseUnit::Second).pow(Number::from_i64(-2)))
                .mul(Unit::base(BaseUnit::Ampere).pow(Number::neg_one()))
        )),
        "henry" | "H" => Some(Quantity::new(Number::from_i64(1000),
            Unit::base(BaseUnit::Gram)
                .mul(Unit::base(BaseUnit::Meter).pow(Number::from_i64(2)))
                .mul(Unit::base(BaseUnit::Second).pow(Number::from_i64(-2)))
                .mul(Unit::base(BaseUnit::Ampere).pow(Number::from_i64(-2)))
        )),
        "lumen" | "lumens" | "lm" => Some(Quantity::new(Number::one(), Unit::base(BaseUnit::Candela))),
        "lux" | "lx" => Some(Quantity::new(Number::one(),
            Unit::base(BaseUnit::Candela).mul(Unit::base(BaseUnit::Meter).pow(Number::from_i64(-2)))
        )),
        "becquerel" | "Bq" => Some(Quantity::new(Number::one(), Unit::base(BaseUnit::Candela).pow(Number::neg_one()))),
        "gray" | "Gy" => Some(Quantity::new(Number::one(),
            Unit::base(BaseUnit::Gram).pow(Number::from_i64(2))
                .mul(Unit::base(BaseUnit::Second).pow(Number::from_i64(-2)))
        )),
        "sievert" | "Sv" => Some(Quantity::new(Number::one(),
            Unit::base(BaseUnit::Gram).pow(Number::from_i64(2))
                .mul(Unit::base(BaseUnit::Second).pow(Number::from_i64(-2)))
        )),
        "katal" | "kat" => Some(Quantity::new(Number::one(),
            Unit::base(BaseUnit::Mole).mul(Unit::base(BaseUnit::Second).pow(Number::neg_one()))
        )),
        // Not SI units, but can be used with SI prefix
        "liter" | "liters" | "litre" | "l" | "L" => Some(Quantity::new(Number::from_i64s(1, 1_000), Unit::base(BaseUnit::Meter).pow(Number::from_i64(3)))),
        "tonne" | "tonnes" | "t" => Some(Quantity::new(Number::from_i64(1_000_000), Unit::base(BaseUnit::Gram))),
        "electronvolt" | "eV" => Some(Quantity::new(Number::from_i128s(1_602_176_634, (10 as i128).pow(25)),
            Unit::base(BaseUnit::Gram)
                .mul(Unit::base(BaseUnit::Meter).pow(Number::from_i64(2)))
                .mul(Unit::base(BaseUnit::Second).pow(Number::from_i64(-2)))
        )),
        _ => None,
    }
}

impl QuantityContext {
    pub fn new() -> QuantityContext {
        QuantityContext { vars: HashMap::new(), funcs: HashMap::new() }
    }

    fn buildin_variable_for(&self, name: &str) -> Option<Quantity> {
        return match name {
            "pi" => Some(Quantity::pi()),
            "e" => Some(Quantity::e()),
            // Units
            // Imperial length
            "twips" | "twip" => Some(Quantity::new(Number::from_i128s(176_389, (10 as i128).pow(10)), Unit::base(BaseUnit::Meter))),
            "thou" | "thous" | "th" => Some(Quantity::new(Number::from_i128s(254, (10 as i128).pow(7)), Unit::base(BaseUnit::Meter))),
            "barleycorn" | "barleycorns" | "Bc" => Some(Quantity::new(Number::from_i128s(84_667, (10 as i128).pow(7)), Unit::base(BaseUnit::Meter))),
            "inch" | "inches" | "in" => Some(Quantity::new(Number::from_i128s(0_0254, (10 as i128).pow(4)), Unit::base(BaseUnit::Meter))),
            "hands" | "hand" => Some(Quantity::new(Number::from_i128s(0_1016, (10 as i128).pow(4)), Unit::base(BaseUnit::Meter))),
            "foot" | "feet" | "ft" => Some(Quantity::new(Number::from_i128s(0_3048, (10 as i128).pow(4)), Unit::base(BaseUnit::Meter))),
            "yard" | "yards" | "yd" => Some(Quantity::new(Number::from_i128s(0_9144, (10 as i128).pow(4)), Unit::base(BaseUnit::Meter))),
            "chain" | "chains" | "ch" => Some(Quantity::new(Number::from_i128s(20_1168, (10 as i128).pow(4)), Unit::base(BaseUnit::Meter))),
            "furlong" | "furlongs" | "fur" => Some(Quantity::new(Number::from_i128s(201_168, (10 as i128).pow(3)), Unit::base(BaseUnit::Meter))),
            "mile" | "miles" | "mi" => Some(Quantity::new(Number::from_i128s(1609_344, (10 as i128).pow(3)), Unit::base(BaseUnit::Meter))),
            "league" | "leagues" | "lea" => Some(Quantity::new(Number::from_i128s(4828_032, (10 as i128).pow(3)), Unit::base(BaseUnit::Meter))),
            "fathom" | "fathoms" | "ftm" => Some(Quantity::new(Number::from_i128s(1_852, (10 as i128).pow(3)), Unit::base(BaseUnit::Meter))),
            "cables" | "cable" => Some(Quantity::new(Number::from_i128s(185_2, 10), Unit::base(BaseUnit::Meter))),
            "nauticalmile" | "nauticalmiles" | "nmi" => Some(Quantity::new(Number::from_i128(1852), Unit::base(BaseUnit::Meter))),
            "links" | "link" => Some(Quantity::new(Number::from_i128s(0_201168, (10 as i128).pow(6)), Unit::base(BaseUnit::Meter))),
            "rods" | "rod" => Some(Quantity::new(Number::from_i128s(5_0292, (10 as i128).pow(4)), Unit::base(BaseUnit::Meter))),
            // Imperial area
            "perches" | "perch" => Some(Quantity::new(
                Number::from_i128s(25_29285264, (10 as i128).pow(8)), Unit::base(BaseUnit::Meter).pow(Number::from_i64(2))
            )),
            "roods" | "rood" => Some(Quantity::new(
                Number::from_i128s(1011_7141056, (10 as i128).pow(7)), Unit::base(BaseUnit::Meter).pow(Number::from_i64(2))
            )),
            "acres" | "acre" => Some(Quantity::new(
                Number::from_i128s(4046_8564224, (10 as i128).pow(7)), Unit::base(BaseUnit::Meter).pow(Number::from_i64(2))
            )),
            "squaremile" | "squaremiles" | "sqmi" => Some(Quantity::new(
                Number::from_i128s(2589988_110336, 1_000_000), Unit::base(BaseUnit::Meter).pow(Number::from_i64(2))
            )),
            // Imperial volume
            "fluidounce" | "fluidounces" | "floz" => Some(Quantity::new(
                Number::from_i128s(28_4130625, (10 as i128).pow(13)), Unit::base(BaseUnit::Meter).pow(Number::from_i64(3))
            )),
            "gill" | "gills" | "gi" => Some(Quantity::new(
                Number::from_i128s(142_0653125, (10 as i128).pow(13)), Unit::base(BaseUnit::Meter).pow(Number::from_i64(3))
            )),
            "pint" | "pints" | "pt" => Some(Quantity::new(
                Number::from_i128s(568_26125, (10 as i128).pow(11)), Unit::base(BaseUnit::Meter).pow(Number::from_i64(3))
            )),
            "quart" | "quarts" | "qt" => Some(Quantity::new(
                Number::from_i128s(1136_5225, (10 as i128).pow(10)), Unit::base(BaseUnit::Meter).pow(Number::from_i64(3))
            )),
            "gallon" | "gallons" | "gal" => Some(Quantity::new(
                Number::from_i128s(4546_09, (10 as i128).pow(8)), Unit::base(BaseUnit::Meter).pow(Number::from_i64(3))
            )),
            // Imperial mass
            "grain" | "grains" | "gr" => Some(Quantity::new(Number::from_i128s(0_06479891, (10 as i128).pow(8)), Unit::base(BaseUnit::Gram))),
            "drachm" | "drachms" | "dr" => Some(Quantity::new(Number::from_i128s(1_7718451953125, (10 as i128).pow(13)), Unit::base(BaseUnit::Gram))),
            "ounce" | "ounces" | "oz" => Some(Quantity::new(Number::from_i128s(28_349523125, (10 as i128).pow(9)), Unit::base(BaseUnit::Gram))),
            "pound" | "pounds" | "lb" => Some(Quantity::new(Number::from_i128s(453_59237, (10 as i128).pow(5)), Unit::base(BaseUnit::Gram))),
            "stone" | "stones" | "st" => Some(Quantity::new(Number::from_i128s(6350_29318, (10 as i128).pow(5)), Unit::base(BaseUnit::Gram))),
            "quarter" | "quarters" | "qr" => Some(Quantity::new(Number::from_i128s(12700_58636, (10 as i128).pow(5)), Unit::base(BaseUnit::Gram))),
            "hundredweight" | "hundredweights" | "cwt" => Some(Quantity::new(Number::from_i128s(50802_34544, (10 as i128).pow(5)), Unit::base(BaseUnit::Gram))),
            "tons" | "ton" => Some(Quantity::new(Number::from_i128s(1016046_9088, (10 as i128).pow(4)), Unit::base(BaseUnit::Gram))),
            "slugs" | "slug"=> Some(Quantity::new(Number::from_i128s(14593_90294, (10 as i128).pow(5)), Unit::base(BaseUnit::Gram))),
            // SI accepted
            "minute" | "minutes" | "min" => Some(Quantity::new(Number::from_i64(60), Unit::base(BaseUnit::Second))),
            "hour" | "hours" | "h" => Some(Quantity::new(Number::from_i64(3600), Unit::base(BaseUnit::Second))),
            "day" | "days" | "d" => Some(Quantity::new(Number::from_i64(86400), Unit::base(BaseUnit::Second))),
            "astronomicalunit" | "au" => Some(Quantity::new(Number::from_i64(149_597_870_700), Unit::base(BaseUnit::Meter))),
            "hectare" | "hectares" | "ha" => Some(Quantity::new(Number::from_i64(10_000), Unit::base(BaseUnit::Meter).pow(Number::from_i64(2)))),
            "dalton" | "Da" => Some(Quantity::new(Number::from_i128s(166_053_904_020, (10 as i128).pow(35)), Unit::base(BaseUnit::Gram))),
            _ => {
                if let Some(val) = get_base_unit_for(name) {
                    return Some(val);
                }
                let (mult, div, len) = match name {
                    _ if name.starts_with("yotta") => (1_000_000_000_000_000_000_000_000, 1, 5),
                    _ if name.starts_with("zetta") => (1_000_000_000_000_000_000_000, 1, 5),
                    _ if name.starts_with("exa") => (1_000_000_000_000_000_000, 1, 3),
                    _ if name.starts_with("peta") => (1_000_000_000_000_000, 1, 4),
                    _ if name.starts_with("tera") => (1_000_000_000_000, 1, 4),
                    _ if name.starts_with("giga") => (1_000_000_000, 1, 4),
                    _ if name.starts_with("mega") => (1_000_000, 1, 4),
                    _ if name.starts_with("kilo") => (1_000, 1, 4),
                    _ if name.starts_with("hecto") => (100, 1, 5),
                    _ if name.starts_with("deca") => (10, 1, 4),
                    _ if name.starts_with("yocto") => (1, 1_000_000_000_000_000_000_000_000, 5),
                    _ if name.starts_with("zepto") => (1, 1_000_000_000_000_000_000_000, 5),
                    _ if name.starts_with("atto") => (1, 1_000_000_000_000_000_000, 4),
                    _ if name.starts_with("femto") => (1, 1_000_000_000_000_000, 5),
                    _ if name.starts_with("pico") => (1, 1_000_000_000_000, 4),
                    _ if name.starts_with("nano") => (1, 1_000_000_000, 4),
                    _ if name.starts_with("micro") => (1, 1_000_000, 5),
                    _ if name.starts_with("milli") => (1, 1_000, 5),
                    _ if name.starts_with("centi") => (1, 100, 5),
                    _ if name.starts_with("deci") => (1, 10, 4),
                    _ if name.starts_with("Y") => (1_000_000_000_000_000_000_000_000, 1, 1),
                    _ if name.starts_with("Z") => (1_000_000_000_000_000_000_000, 1, 1),
                    _ if name.starts_with("E") => (1_000_000_000_000_000_000, 1, 1),
                    _ if name.starts_with("P") => (1_000_000_000_000_000, 1, 1),
                    _ if name.starts_with("T") => (1_000_000_000_000, 1, 1),
                    _ if name.starts_with("G") => (1_000_000_000, 1, 1),
                    _ if name.starts_with("M") => (1_000_000, 1, 1),
                    _ if name.starts_with("k") => (1_000, 1, 1),
                    _ if name.starts_with("h") => (100, 1, 1),
                    _ if name.starts_with("da") => (10, 1, 2),
                    _ if name.starts_with("y") => (1, 1_000_000_000_000_000_000_000_000, 1),
                    _ if name.starts_with("z") => (1, 1_000_000_000_000_000_000_000, 1),
                    _ if name.starts_with("a") => (1, 1_000_000_000_000_000_000, 1),
                    _ if name.starts_with("f") => (1, 1_000_000_000_000_000, 1),
                    _ if name.starts_with("p") => (1, 1_000_000_000_000, 1),
                    _ if name.starts_with("n") => (1, 1_000_000_000, 1),
                    _ if name.starts_with("u") => (1, 1_000_000, 1),
                    _ if name.starts_with("m") => (1, 1_000, 1),
                    _ if name.starts_with("c") => (1, 100, 1),
                    _ if name.starts_with("d") => (1, 10, 1),
                    _ => (1, 1, 0),
                };
                let prefix = Number::from_i128s(mult, div);
                if len < name.len() && len != 0 {
                    if let Some(val) = get_base_unit_for(&name[len..]) {
                        return Some((val * Quantity::unitless(prefix)).unwrap());
                    }
                }
                return None;
            }
        }
    }

    fn buildin_function_for(&self, name: &str) -> Option<&'static ContextFn<Quantity>> {
        match name {
            "abs" => Some(&|v| check_length(v, 1, 1)?[0].abs().nan_to_err()),
            "sign" => Some(&|v| check_length(v, 1, 1)?[0].sign().nan_to_err()),
            "sqrt" => Some(&|v| check_length(v, 1, 1)?[0].sqrt().nan_to_err()),
            "cbrt" => Some(&|v| check_length(v, 1, 1)?[0].cbrt().nan_to_err()),
            "min" => Some(&min),
            "max" => Some(&max),
            "floor" => Some(&|v| unitless_function(v, Number::floor)),
            "ceil" => Some(&|v| unitless_function(v, Number::ceil)),
            "round" => Some(&|v| unitless_function(v, Number::round)),
            "trunc" => Some(&|v| unitless_function(v, Number::trunc)),
            "fract" => Some(&|v| unitless_function(v, Number::fract)),
            "ln" => Some(&|v| unitless_function(v, Number::ln)),
            "log" => Some(&|v| unitless_function(v, Number::log)),
            "sin" => Some(&|v| unitless_function(v, Number::sin)),
            "cos" => Some(&|v| unitless_function(v, Number::cos)),
            "tan" => Some(&|v| unitless_function(v, Number::tan)),
            "asin" => Some(&|v| unitless_function(v, Number::asin)),
            "acos" => Some(&|v| unitless_function(v, Number::acos)),
            "atan" => Some(&|v| unitless_function(v, Number::atan)),
            "atan2" => Some(&|mut v| {
                v = check_length(v, 2, 2)?;
                if v[0].is_unitless() && v[1].is_unitless() {
                    let num = v[0].coefficient().atan2(v[1].coefficient());
                    return num.nan_to_err().map(|x| Quantity::unitless(x));
                } else {
                    return Err(EvalError::UnitError("Function can only be applied to unitless quantity".to_owned()));
                }
            }),
            "sinh" => Some(&|v| unitless_function(v, Number::sinh)),
            "cosh" => Some(&|v| unitless_function(v, Number::cosh)),
            "tanh" => Some(&|v| unitless_function(v, Number::tanh)),
            "asinh" => Some(&|v| unitless_function(v, Number::asinh)),
            "acosh" => Some(&|v| unitless_function(v, Number::acosh)),
            "atanh" => Some(&|v| unitless_function(v, Number::atanh)),
            _ => None,
        }
    }
}

impl Context<Quantity> for QuantityContext {
    fn set_variable(&mut self, name: &str, value: Quantity) {
        self.vars.insert(name.to_owned(), value);
    }

    fn set_function(&mut self, name: &str, value: Box<ContextFn<Quantity>>) {
        self.funcs.insert(name.to_owned(), value);
    }

    fn get_variable(&self, name: &str) -> Option<Quantity> {
        self.vars.get(name)
            .and_then(|n| Some(n.clone()))
            .or_else(|| self.buildin_variable_for(name))
    }

    fn get_function<'a>(&'a self, name: &str) -> Option<&'a ContextFn<Quantity>> {
        self.funcs.get(name)
            .and_then(|n| Some(n.as_ref()))
            .or_else(|| self.buildin_function_for(name))
    }
}

