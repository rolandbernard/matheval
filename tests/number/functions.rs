
use std::str::FromStr;

use matheval::Number;

#[test]
fn floor_rational() {
    assert_eq!("12", Number::from_str("12.46").unwrap().floor().to_string());
    assert_eq!("12", Number::from_str("12.76").unwrap().floor().to_string());
    assert_eq!("-13", Number::from_str("-12.46").unwrap().floor().to_string());
    assert_eq!("-13", Number::from_str("-12.76").unwrap().floor().to_string());
}

#[test]
fn floor_float() {
    assert_eq!("12", Number::Float(12.46).floor().to_string());
    assert_eq!("12", Number::Float(12.76).floor().to_string());
    assert_eq!("-13", Number::Float(-12.46).floor().to_string());
    assert_eq!("-13", Number::Float(-12.76).floor().to_string());
}

#[test]
fn ceil_rational() {
    assert_eq!("13", Number::from_str("12.46").unwrap().ceil().to_string());
    assert_eq!("13", Number::from_str("12.76").unwrap().ceil().to_string());
    assert_eq!("-12", Number::from_str("-12.46").unwrap().ceil().to_string());
    assert_eq!("-12", Number::from_str("-12.76").unwrap().ceil().to_string());
}

#[test]
fn ceil_float() {
    assert_eq!("13", Number::Float(12.46).ceil().to_string());
    assert_eq!("13", Number::Float(12.76).ceil().to_string());
    assert_eq!("-12", Number::Float(-12.46).ceil().to_string());
    assert_eq!("-12", Number::Float(-12.76).ceil().to_string());
}

#[test]
fn round_rational() {
    assert_eq!("12", Number::from_str("12.46").unwrap().round().to_string());
    assert_eq!("13", Number::from_str("12.76").unwrap().round().to_string());
    assert_eq!("-12", Number::from_str("-12.46").unwrap().round().to_string());
    assert_eq!("-13", Number::from_str("-12.76").unwrap().round().to_string());
}

#[test]
fn round_float() {
    assert_eq!("12", Number::Float(12.46).round().to_string());
    assert_eq!("13", Number::Float(12.76).round().to_string());
    assert_eq!("-12", Number::Float(-12.46).round().to_string());
    assert_eq!("-13", Number::Float(-12.76).round().to_string());
}

#[test]
fn trunc_rational() {
    assert_eq!("12", Number::from_str("12.46").unwrap().trunc().to_string());
    assert_eq!("12", Number::from_str("12.76").unwrap().trunc().to_string());
    assert_eq!("-12", Number::from_str("-12.46").unwrap().trunc().to_string());
    assert_eq!("-12", Number::from_str("-12.76").unwrap().trunc().to_string());
}

#[test]
fn trunc_float() {
    assert_eq!("12", Number::Float(12.46).trunc().to_string());
    assert_eq!("12", Number::Float(12.76).trunc().to_string());
    assert_eq!("-12", Number::Float(-12.46).trunc().to_string());
    assert_eq!("-12", Number::Float(-12.76).trunc().to_string());
}

#[test]
fn fract_rational() {
    assert_eq!("23/50", Number::from_str("12.46").unwrap().fract().to_string());
    assert_eq!("19/25", Number::from_str("12.76").unwrap().fract().to_string());
    assert_eq!("-23/50", Number::from_str("-12.46").unwrap().fract().to_string());
    assert_eq!("-19/25", Number::from_str("-12.76").unwrap().fract().to_string());
}

#[test]
fn fract_float() {
    assert_eq!("0.75", Number::Float(12.75).fract().to_string());
    assert_eq!("-0.75", Number::Float(-12.75).fract().to_string());
}

#[test]
fn abs_rational() {
    assert_eq!("623/50", Number::from_str("12.46").unwrap().abs().to_string());
    assert_eq!("319/25", Number::from_str("-12.76").unwrap().abs().to_string());
}

#[test]
fn abs_float() {
    assert_eq!("12.46", Number::Float(12.46).abs().to_string());
    assert_eq!("12.76", Number::Float(-12.76).abs().to_string());
}

#[test]
fn sign_rational() {
    assert_eq!("1", Number::from_str("12.46").unwrap().sign().to_string());
    assert_eq!("-1", Number::from_str("-12.76").unwrap().sign().to_string());
    assert_eq!("0", Number::from_str("0.0").unwrap().sign().to_string());
}

#[test]
fn sign_float() {
    assert_eq!("1", Number::Float(12.46).sign().to_string());
    assert_eq!("-1", Number::Float(-12.76).sign().to_string());
    assert_eq!("0", Number::Float(0.0).sign().to_string());
}

#[test]
fn sqrt_rational() {
    assert_eq!(Number::from_str("1.6").unwrap(), Number::from_str("2.56").unwrap().sqrt());
    assert_eq!(Number::zero(), Number::from_str("0.0").unwrap().sqrt());
    assert!(Number::from_str("2.56").unwrap().sqrt().is_rational());
    assert!(Number::from_str("64").unwrap().sqrt().is_integer());
}

#[test]
fn sqrt_float() {
    assert_eq!("3.529872518944558", Number::from_str("12.46").unwrap().sqrt().to_string());
    assert_eq!("0", Number::from_str("0.0").unwrap().sqrt().to_string());
    assert_eq!("3.529872518944558", Number::Float(12.46).sqrt().to_string());
    assert_eq!("0", Number::Float(0.0).sqrt().to_string());
}

#[test]
fn ln_float() {
    assert_eq!("2.522523513359307", Number::from_str("12.46").unwrap().ln().to_string());
    assert_eq!("0", Number::from_str("1.0").unwrap().ln().to_string());
    assert_eq!("2.522523513359307", Number::Float(12.46).ln().to_string());
    assert_eq!("0", Number::Float(1.0).ln().to_string());
}

#[test]
fn log_float() {
    assert_eq!("1.0955180423231508", Number::from_str("12.46").unwrap().log().to_string());
    assert_eq!("0", Number::from_str("1.0").unwrap().log().to_string());
    assert_eq!("1.0955180423231508", Number::Float(12.46).log().to_string());
    assert_eq!("0", Number::Float(1.0).log().to_string());
}

#[test]
fn cbrt_rational() {
    assert_eq!(Number::from_str("1.6").unwrap(), Number::from_str("4.096").unwrap().cbrt());
    assert_eq!(Number::zero(), Number::from_str("0.0").unwrap().cbrt());
    assert!(Number::from_str("4.096").unwrap().cbrt().is_rational());
    assert!(Number::from_str("512").unwrap().cbrt().is_integer());
}

#[test]
fn cbrt_float() {
    assert_eq!("2.3183162575091356", Number::from_str("12.46").unwrap().cbrt().to_string());
    assert_eq!("0", Number::from_str("0.0").unwrap().cbrt().to_string());
    assert_eq!("2.3183162575091356", Number::Float(12.46).cbrt().to_string());
    assert_eq!("0", Number::Float(0.0).cbrt().to_string());
}

#[test]
fn sin_float() {
    assert_eq!("0.5167068002272901", Number::from_str("0.543").unwrap().sin().to_string());
    assert_eq!("0", Number::from_str("0.0").unwrap().sin().to_string());
    assert_eq!("0.5167068002272901", Number::Float(0.543).sin().to_string());
    assert_eq!("0", Number::Float(0.0).sin().to_string());
}

#[test]
fn cos_float() {
    assert_eq!("0.8561624160163044", Number::from_str("0.543").unwrap().cos().to_string());
    assert_eq!("1", Number::from_str("0.0").unwrap().cos().to_string());
    assert_eq!("0.8561624160163044", Number::Float(0.543).cos().to_string());
    assert_eq!("1", Number::Float(0.0).cos().to_string());
}

#[test]
fn tan_float() {
    assert_eq!("0.6035149295988836", Number::from_str("0.543").unwrap().tan().to_string());
    assert_eq!("0", Number::from_str("0.0").unwrap().tan().to_string());
    assert_eq!("0.6035149295988836", Number::Float(0.543).tan().to_string());
    assert_eq!("0", Number::Float(0.0).tan().to_string());
}

#[test]
fn asin_float() {
    assert_eq!("0.5740055653279919", Number::from_str("0.543").unwrap().asin().to_string());
    assert_eq!("0", Number::from_str("0.0").unwrap().asin().to_string());
    assert_eq!("0.5740055653279919", Number::Float(0.543).asin().to_string());
    assert_eq!("0", Number::Float(0.0).asin().to_string());
}

#[test]
fn acos_float() {
    assert_eq!("0.9967907614669048", Number::from_str("0.543").unwrap().acos().to_string());
    assert_eq!("1.5707963267948966", Number::from_str("0.0").unwrap().acos().to_string());
    assert_eq!("0.9967907614669048", Number::Float(0.543).acos().to_string());
    assert_eq!("1.5707963267948966", Number::Float(0.0).acos().to_string());
}

#[test]
fn atan_float() {
    assert_eq!("0.49745305021666625", Number::from_str("0.543").unwrap().atan().to_string());
    assert_eq!("0", Number::from_str("0.0").unwrap().atan().to_string());
    assert_eq!("0.49745305021666625", Number::Float(0.543).atan().to_string());
    assert_eq!("0", Number::Float(0.0).atan().to_string());
}

#[test]
fn atan2_float() {
    assert_eq!("0.49745305021666625", Number::from_str("0.543").unwrap().atan2(&Number::one()).to_string());
    assert_eq!("0", Number::from_str("0.0").unwrap().atan2(&Number::one()).to_string());
    assert_eq!("0.49745305021666625", Number::Float(0.543).atan2(&Number::one()).to_string());
    assert_eq!("0", Number::Float(0.0).atan2(&Number::one()).to_string());
}

#[test]
fn sinh_float() {
    assert_eq!("0.5700799925832585", Number::from_str("0.543").unwrap().sinh().to_string());
    assert_eq!("0", Number::from_str("0.0").unwrap().sinh().to_string());
    assert_eq!("0.5700799925832585", Number::Float(0.543).sinh().to_string());
    assert_eq!("0", Number::Float(0.0).sinh().to_string());
}

#[test]
fn cosh_float() {
    assert_eq!("1.1510826199468602", Number::from_str("0.543").unwrap().cosh().to_string());
    assert_eq!("1", Number::from_str("0.0").unwrap().cosh().to_string());
    assert_eq!("1.1510826199468602", Number::Float(0.543).cosh().to_string());
    assert_eq!("1", Number::Float(0.0).cosh().to_string());
}

#[test]
fn tanh_float() {
    assert_eq!("0.49525549487453496", Number::from_str("0.543").unwrap().tanh().to_string());
    assert_eq!("0", Number::from_str("0.0").unwrap().tanh().to_string());
    assert_eq!("0.49525549487453496", Number::Float(0.543).tanh().to_string());
    assert_eq!("0", Number::Float(0.0).tanh().to_string());
}

#[test]
fn asinh_float() {
    assert_eq!("0.5193378835551655", Number::from_str("0.543").unwrap().asinh().to_string());
    assert_eq!("0", Number::from_str("0.0").unwrap().asinh().to_string());
    assert_eq!("0.5193378835551655", Number::Float(0.543).asinh().to_string());
    assert_eq!("0", Number::Float(0.0).asinh().to_string());
}

#[test]
fn acosh_float() {
    assert_eq!("0.999931383282944", Number::from_str("1.543").unwrap().acosh().to_string());
    assert_eq!("0", Number::from_str("1.0").unwrap().acosh().to_string());
    assert_eq!("0.999931383282944", Number::Float(1.543).acosh().to_string());
    assert_eq!("0", Number::Float(1.0).acosh().to_string());
}

#[test]
fn atanh_float() {
    assert_eq!("0.6084002307344781", Number::from_str("0.543").unwrap().atanh().to_string());
    assert_eq!("0", Number::from_str("0.0").unwrap().atanh().to_string());
    assert_eq!("0.6084002307344781", Number::Float(0.543).atanh().to_string());
    assert_eq!("0", Number::Float(0.0).atanh().to_string());
}

