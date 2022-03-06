
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
    pub fn symbol(&self) -> &'static str {
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
            let symbol = BaseUnit::try_from(i).unwrap().symbol();
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

