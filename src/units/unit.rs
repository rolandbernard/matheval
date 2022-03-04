
use std::ops::*;
use num::traits::Pow;

use crate::Number;

const BASE_UNIT_COUNT: usize = 6;

pub enum BaseUnit {
    Second = 0,
    Meter,
    Kilogram,
    Ampere,
    Mole,
    Kelvin,
    Candela
}

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

