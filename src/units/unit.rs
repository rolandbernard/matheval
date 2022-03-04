
use std::ops::*;
use num::traits::Pow;

use crate::Number;

const BASE_UNIT_COUNT: usize = 6;

#[derive(Debug, Clone)]
pub enum BaseUnit {
    Second = 0,
    Meter,
    Gram,
    Ampere,
    Mole,
    Kelvin,
    Candela
}

impl TryFrom<usize> for BaseUnit {
    type Error = ();

    fn try_from(v: usize) -> Result<Self, Self::Error> {
        match v {
            x if x == BaseUnit::Second as usize => Ok(BaseUnit::Second),
            x if x == BaseUnit::Meter as usize => Ok(BaseUnit::Meter),
            x if x == BaseUnit::Gram as usize => Ok(BaseUnit::Gram),
            x if x == BaseUnit::Ampere as usize => Ok(BaseUnit::Ampere),
            x if x == BaseUnit::Mole as usize => Ok(BaseUnit::Mole),
            x if x == BaseUnit::Kelvin as usize => Ok(BaseUnit::Kelvin),
            x if x == BaseUnit::Candela as usize => Ok(BaseUnit::Candela),
            _ => Err(()),
        }
    }
}

impl BaseUnit {
    pub fn all() -> Vec<BaseUnit> {
        vec![
            BaseUnit::Second, BaseUnit::Meter,
            BaseUnit::Gram, BaseUnit::Ampere,
            BaseUnit::Mole, BaseUnit::Kelvin,
            BaseUnit::Candela,
        ]
    }

    pub fn base_symbol(&self) -> &'static str {
        match self {
            BaseUnit::Second => "s",
            BaseUnit::Meter => "m",
            BaseUnit::Gram => "g",
            BaseUnit::Ampere => "A",
            BaseUnit::Mole => "mol",
            BaseUnit::Kelvin => "K",
            BaseUnit::Candela => "cd",
        }
    }

    pub fn all_prefix() -> Vec<i64> {
        vec![-24, -21, -18, -15, -12, -9, -6, -3, -2, -1, 0, 1, 2, 3, 6, 9, 12, 15, 18, 21, 24]
    }

    pub fn prefix_symbol(power: i64) -> Option<&'static str> {
        match power {
            -24 => Some("y"), -21 => Some("z"),
            -18 => Some("a"), -15 => Some("f"),
            -12 => Some("p"), -9 => Some("n"),
            -6 => Some("u"), -3 => Some("m"),
            -2 => Some("c"), -1 => Some("d"),
            0 => Some(""), 1 => Some("da"),
            2 => Some("h"), 3 => Some("k"),
            6 => Some("M"), 9 => Some("G"),
            12 => Some("T"), 15 => Some("P"),
            18 => Some("E"), 21 => Some("Z"),
            24 => Some("Y"), _ => None,
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Unit {
    units: Vec<Number>,
}

impl Unit {
    pub fn empty() -> Unit {
        Unit { units: vec![Number::zero(); BASE_UNIT_COUNT] }
    }

    pub fn base(unit: BaseUnit) -> Unit {
        let mut units = vec![Number::zero(); BASE_UNIT_COUNT];
        units[unit as usize] = Number::one();
        Unit { units }
    }

    pub fn is_empty(&self) -> bool {
        self.units.iter().all(|n| n.is_zero())
    }
}

impl ToString for Unit {
    fn to_string(&self) -> String {
        let mut ret = String::new();
        for (i, v) in self.units.iter().enumerate() {
            if i != 0 {
                ret.push(' ');
            }
            let symbol = BaseUnit::try_from(i).unwrap().base_symbol();
            if v.is_positive() || v.is_integer() || !v.is_rational() {
                ret.push_str(&format!("{}^{}", symbol, v.to_string()));
            } else {
                ret.push_str(&format!("{}^({})", symbol, v.to_string()));
            }
        }
        return ret;
    }
}

impl Mul for Unit {
    type Output = Unit;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut units = Vec::new();
        for (l, r) in self.units.into_iter().zip(rhs.units.into_iter()) {
            units.push((l + r).unwrap());
        }
        Unit { units }
    }
}

impl Div for Unit {
    type Output = Unit;

    fn div(self, rhs: Self) -> Self::Output {
        let mut units = Vec::new();
        for (l, r) in self.units.into_iter().zip(rhs.units.into_iter()) {
            units.push((l - r).unwrap());
        }
        Unit { units }
    }
}

impl Pow<Number> for Unit {
    type Output = Unit;

    fn pow(self, rhs: Number) -> Self::Output {
        let mut units = Vec::new();
        for l in self.units {
            units.push((l + rhs.clone()).unwrap());
        }
        Unit { units }
    }
}

