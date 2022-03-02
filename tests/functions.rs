
use std::str::FromStr;

use matheval::Number;

#[test]
fn function_floor_rational() {
    assert_eq!("12", Number::from_str("12.46").unwrap().floor().to_string());
    assert_eq!("12", Number::from_str("12.76").unwrap().floor().to_string());
    assert_eq!("-13", Number::from_str("-12.46").unwrap().floor().to_string());
    assert_eq!("-13", Number::from_str("-12.76").unwrap().floor().to_string());
}

#[test]
fn function_floor_float() {
    assert_eq!("12", Number::Float(12.46).floor().to_string());
    assert_eq!("12", Number::Float(12.76).floor().to_string());
    assert_eq!("-13", Number::Float(-12.46).floor().to_string());
    assert_eq!("-13", Number::Float(-12.76).floor().to_string());
}

#[test]
fn function_ceil_rational() {
    assert_eq!("13", Number::from_str("12.46").unwrap().ceil().to_string());
    assert_eq!("13", Number::from_str("12.76").unwrap().ceil().to_string());
    assert_eq!("-12", Number::from_str("-12.46").unwrap().ceil().to_string());
    assert_eq!("-12", Number::from_str("-12.76").unwrap().ceil().to_string());
}

#[test]
fn function_ceil_float() {
    assert_eq!("13", Number::Float(12.46).ceil().to_string());
    assert_eq!("13", Number::Float(12.76).ceil().to_string());
    assert_eq!("-12", Number::Float(-12.46).ceil().to_string());
    assert_eq!("-12", Number::Float(-12.76).ceil().to_string());
}

#[test]
fn function_round_rational() {
    assert_eq!("12", Number::from_str("12.46").unwrap().round().to_string());
    assert_eq!("13", Number::from_str("12.76").unwrap().round().to_string());
    assert_eq!("-12", Number::from_str("-12.46").unwrap().round().to_string());
    assert_eq!("-13", Number::from_str("-12.76").unwrap().round().to_string());
}

#[test]
fn function_round_float() {
    assert_eq!("12", Number::Float(12.46).round().to_string());
    assert_eq!("13", Number::Float(12.76).round().to_string());
    assert_eq!("-12", Number::Float(-12.46).round().to_string());
    assert_eq!("-13", Number::Float(-12.76).round().to_string());
}

#[test]
fn function_trunc_rational() {
    assert_eq!("12", Number::from_str("12.46").unwrap().trunc().to_string());
    assert_eq!("12", Number::from_str("12.76").unwrap().trunc().to_string());
    assert_eq!("-12", Number::from_str("-12.46").unwrap().trunc().to_string());
    assert_eq!("-12", Number::from_str("-12.76").unwrap().trunc().to_string());
}

#[test]
fn function_trunc_float() {
    assert_eq!("12", Number::Float(12.46).trunc().to_string());
    assert_eq!("12", Number::Float(12.76).trunc().to_string());
    assert_eq!("-12", Number::Float(-12.46).trunc().to_string());
    assert_eq!("-12", Number::Float(-12.76).trunc().to_string());
}

#[test]
fn function_fract_rational() {
    assert_eq!("23/50", Number::from_str("12.46").unwrap().fract().to_string());
    assert_eq!("19/25", Number::from_str("12.76").unwrap().fract().to_string());
    assert_eq!("-23/50", Number::from_str("-12.46").unwrap().fract().to_string());
    assert_eq!("-19/25", Number::from_str("-12.76").unwrap().fract().to_string());
}

#[test]
fn function_fract_float() {
    assert_eq!("0.75", Number::Float(12.75).fract().to_string());
    assert_eq!("-0.75", Number::Float(-12.75).fract().to_string());
}

#[test]
fn function_abs_rational() {
    assert_eq!("623/50", Number::from_str("12.46").unwrap().abs().to_string());
    assert_eq!("319/25", Number::from_str("-12.76").unwrap().abs().to_string());
}

#[test]
fn function_abs_float() {
    assert_eq!("12.46", Number::Float(12.46).abs().to_string());
    assert_eq!("12.76", Number::Float(-12.76).abs().to_string());
}

#[test]
fn function_sign_rational() {
    assert_eq!("1", Number::from_str("12.46").unwrap().sign().to_string());
    assert_eq!("-1", Number::from_str("-12.76").unwrap().sign().to_string());
    assert_eq!("0", Number::from_str("0.0").unwrap().sign().to_string());
}

#[test]
fn function_sign_float() {
    assert_eq!("1", Number::Float(12.46).sign().to_string());
    assert_eq!("-1", Number::Float(-12.76).sign().to_string());
    assert_eq!("0", Number::Float(0.0).sign().to_string());
}

#[test]
fn function_sqrt_float() {
    assert_eq!("3.529872518944558", Number::from_str("12.46").unwrap().sqrt().to_string());
    assert_eq!("0", Number::from_str("0.0").unwrap().sqrt().to_string());
    assert_eq!("3.529872518944558", Number::Float(12.46).sqrt().to_string());
    assert_eq!("0", Number::Float(0.0).sqrt().to_string());
}

#[test]
fn function_ln_float() {
    assert_eq!("2.522523513359307", Number::from_str("12.46").unwrap().ln().to_string());
    assert_eq!("0", Number::from_str("1.0").unwrap().ln().to_string());
    assert_eq!("2.522523513359307", Number::Float(12.46).ln().to_string());
    assert_eq!("0", Number::Float(1.0).ln().to_string());
}

#[test]
fn function_log_float() {
    assert_eq!("1.0955180423231508", Number::from_str("12.46").unwrap().log().to_string());
    assert_eq!("0", Number::from_str("1.0").unwrap().log().to_string());
    assert_eq!("1.0955180423231508", Number::Float(12.46).log().to_string());
    assert_eq!("0", Number::Float(1.0).log().to_string());
}

#[test]
fn function_cbrt_float() {
    assert_eq!("2.3183162575091356", Number::from_str("12.46").unwrap().cbrt().to_string());
    assert_eq!("0", Number::from_str("0.0").unwrap().cbrt().to_string());
    assert_eq!("2.3183162575091356", Number::Float(12.46).cbrt().to_string());
    assert_eq!("0", Number::Float(0.0).cbrt().to_string());
}
// sin
// cos
// tan
// asin
// acos
// atan
// atan2
// sinh
// cosh
// tanh
// asinh
// acosh
// atanh

