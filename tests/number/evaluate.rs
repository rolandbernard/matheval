
use std::ops::Add;
use std::str::FromStr;

use matheval::Context;
use matheval::Expr;
use matheval::Number;
use matheval::NumberContext;

#[test]
fn simple_integer_literal() {
    let parsed = Expr::parse("98765432109876543210")
        .expect("Failed to parse simple integer literal");
    assert_eq!(
        Number::from_str("98765432109876543210").expect("Failed parsing number"),
        parsed.eval().expect("Evaluation failed")
    );
}

#[test]
fn binary_integer_literal() {
    let parsed = Expr::parse("0b1001")
        .expect("Failed to parse simple integer literal");
    assert_eq!(
        Number::from_str("9").expect("Failed parsing number"),
        parsed.eval().expect("Evaluation failed")
    );
}

#[test]
fn octal_integer_literal() {
    let parsed = Expr::parse("0o12017")
        .expect("Failed to parse simple integer literal");
    assert_eq!(
        Number::from_str("5135").expect("Failed parsing number"),
        parsed.eval().expect("Evaluation failed")
    );
}

#[test]
fn hex_integer_literal() {
    let parsed = Expr::parse("0xfa0c")
        .expect("Failed to parse simple integer literal");
    assert_eq!(
        Number::from_str("64012").expect("Failed parsing number"),
        parsed.eval().expect("Evaluation failed")
    );
}

#[test]
fn nonint_literal() {
    let parsed = Expr::parse("12.56")
        .expect("Failed to parse simple integer literal");
    assert_eq!(
        Number::from_str("12.56").expect("Failed parsing number"),
        parsed.eval().expect("Evaluation failed")
    );
}

#[test]
fn exp_literal() {
    let parsed = Expr::parse("12.56e-10")
        .expect("Failed to parse simple integer literal");
    assert_eq!(
        Number::from_str("12.56e-10").expect("Failed parsing number"),
        parsed.eval().expect("Evaluation failed")
    );
    let parsed = Expr::parse("12.56e+42")
        .expect("Failed to parse simple integer literal");
    assert_eq!(
        Number::from_str("12.56e42").expect("Failed parsing number"),
        parsed.eval().expect("Evaluation failed")
    );
}

#[test]
fn simple_add() {
    let parsed = Expr::parse("5 + 6e-5 + 42.5")
        .expect("Failed to parse simple integer literal");
    assert_eq!(
        Number::from_str("47.50006").expect("Failed parsing number"),
        parsed.eval().expect("Evaluation failed")
    );
}

#[test]
fn simple_sub() {
    let parsed = Expr::parse("5e2 - 6 - 42.5")
        .expect("Failed to parse simple integer literal");
    assert_eq!(
        Number::from_str("451.5").expect("Failed parsing number"),
        parsed.eval().expect("Evaluation failed")
    );
}

#[test]
fn mixed_add_sub() {
    let parsed = Expr::parse("5 + 6e-5 - 42.5 + 15.7")
        .expect("Failed to parse simple integer literal");
    assert_eq!(
        Number::from_str("-21.79994").expect("Failed parsing number"),
        parsed.eval().expect("Evaluation failed")
    );
}

#[test]
fn simple_mul() {
    let parsed = Expr::parse("5 * 6e-5 * 42.5")
        .expect("Failed to parse simple integer literal");
    assert_eq!(
        Number::from_str("0.01275").expect("Failed parsing number"),
        parsed.eval().expect("Evaluation failed")
    );
}

#[test]
fn simple_div() {
    let parsed = Expr::parse("5e2 / 6 / 42.5")
        .expect("Failed to parse simple integer literal");
    assert_eq!("100/51", parsed.eval::<Number>().expect("Evaluation failed").to_string());
}

#[test]
fn mixed_mul_div() {
    let parsed = Expr::parse("5 / 6e-5 + 42.5 / 15")
        .expect("Failed to parse simple integer literal");
    assert_eq!("500017/6", parsed.eval::<Number>().expect("Evaluation failed").to_string());
}

#[test]
fn simple_pow() {
    let parsed = Expr::parse("2 ^ 3 ^ 2")
        .expect("Failed to parse simple integer literal");
    assert_eq!("512", parsed.eval::<Number>().expect("Evaluation failed").to_string());
    let parsed = Expr::parse("1.5 ^ 3 ^ 2")
        .expect("Failed to parse simple integer literal");
    assert_eq!("19683/512", parsed.eval::<Number>().expect("Evaluation failed").to_string());
    let parsed = Expr::parse("1.5 ^ 3.2 ^ 2.6")
        .expect("Failed to parse simple integer literal");
    assert_eq!("4202.383025252178", parsed.eval::<Number>().expect("Evaluation failed").to_string());
}

#[test]
fn mixed_mul_div_pow() {
    let parsed = Expr::parse("5 / 6^4 + 41^2 / 2^13")
        .expect("Failed to parse simple integer literal");
    assert_eq!("138721/663552", parsed.eval::<Number>().expect("Evaluation failed").to_string());
}

#[test]
fn mixed_parens() {
    let parsed = Expr::parse("(5 / 6)^(4 + 4)1^(2 / 2)^13")
        .expect("Failed to parse simple integer literal");
    assert_eq!("390625/1679616", parsed.eval::<Number>().expect("Evaluation failed").to_string());
}

#[test]
fn functions_general() {
    assert_eq!("2",
        Expr::parse("floor(20/7)").expect("Failed to parse simple integer literal")
            .eval::<Number>().expect("Evaluation failed").to_string()
    );
    assert_eq!("-3",
        Expr::parse("floor(-20/7)").expect("Failed to parse simple integer literal")
            .eval::<Number>().expect("Evaluation failed").to_string()
    );
    assert_eq!("-2",
        Expr::parse("ceil(-15/7)").expect("Failed to parse simple integer literal")
            .eval::<Number>().expect("Evaluation failed").to_string()
    );
    assert_eq!("2",
        Expr::parse("round(15/7)").expect("Failed to parse simple integer literal")
            .eval::<Number>().expect("Evaluation failed").to_string()
    );
    assert_eq!("3",
        Expr::parse("round(20/7)").expect("Failed to parse simple integer literal")
            .eval::<Number>().expect("Evaluation failed").to_string()
    );
    assert_eq!("2",
        Expr::parse("trunc(20/7)").expect("Failed to parse simple integer literal")
            .eval::<Number>().expect("Evaluation failed").to_string()
    );
    assert_eq!("-2",
        Expr::parse("trunc(-20/7)").expect("Failed to parse simple integer literal")
            .eval::<Number>().expect("Evaluation failed").to_string()
    );
    assert_eq!("6/7",
        Expr::parse("fract(20/7)").expect("Failed to parse simple integer literal")
            .eval::<Number>().expect("Evaluation failed").to_string()
    );
    assert_eq!("-6/7",
        Expr::parse("fract(-20/7)").expect("Failed to parse simple integer literal")
            .eval::<Number>().expect("Evaluation failed").to_string()
    );
    assert_eq!("20/7",
        Expr::parse("abs(-20/7)").expect("Failed to parse simple integer literal")
            .eval::<Number>().expect("Evaluation failed").to_string()
    );
    assert_eq!("20/7",
        Expr::parse("abs(20/7)").expect("Failed to parse simple integer literal")
            .eval::<Number>().expect("Evaluation failed").to_string()
    );
    assert_eq!("-1",
        Expr::parse("sign(-20/7)").expect("Failed to parse simple integer literal")
            .eval::<Number>().expect("Evaluation failed").to_string()
    );
    assert_eq!("0",
        Expr::parse("sign(0)").expect("Failed to parse simple integer literal")
            .eval::<Number>().expect("Evaluation failed").to_string()
    );
    assert_eq!("1",
        Expr::parse("sign(20/7)").expect("Failed to parse simple integer literal")
            .eval::<Number>().expect("Evaluation failed").to_string()
    );
    assert_eq!("1.6903085094570331",
        Expr::parse("sqrt(20/7)").expect("Failed to parse simple integer literal")
            .eval::<Number>().expect("Evaluation failed").to_string()
    );
    assert!(
        Expr::parse("sqrt(-20/7)").expect("Failed to parse simple integer literal")
            .eval::<Number>().is_err()
    );
    assert_eq!("1.4189834119703841",
        Expr::parse("cbrt(20/7)").expect("Failed to parse simple integer literal")
            .eval::<Number>().expect("Evaluation failed").to_string()
    );
    assert_eq!("-1.4189834119703841",
        Expr::parse("cbrt(-20/7)").expect("Failed to parse simple integer literal")
            .eval::<Number>().expect("Evaluation failed").to_string()
    );
    assert_eq!("1.0498221244986776",
        Expr::parse("ln(20/7)").expect("Failed to parse simple integer literal")
            .eval::<Number>().expect("Evaluation failed").to_string()
    );
    assert_eq!("0.45593195564972433",
        Expr::parse("log(20/7)").expect("Failed to parse simple integer literal")
            .eval::<Number>().expect("Evaluation failed").to_string()
    );
}

#[test]
fn functions_trig() {
    assert_eq!("0.28062939951435684",
        Expr::parse("sin(20/7)").expect("Failed to parse simple integer literal")
            .eval::<Number>().expect("Evaluation failed").to_string()
    );
    assert_eq!("-0.95981620122199",
        Expr::parse("cos(20/7)").expect("Failed to parse simple integer literal")
            .eval::<Number>().expect("Evaluation failed").to_string()
    );
    assert_eq!("-0.2923782690447124",
        Expr::parse("tan(20/7)").expect("Failed to parse simple integer literal")
            .eval::<Number>().expect("Evaluation failed").to_string()
    );
    assert_eq!("0.7956029534845354",
        Expr::parse("asin(5/7)").expect("Failed to parse simple integer literal")
            .eval::<Number>().expect("Evaluation failed").to_string()
    );
    assert_eq!("0.7751933733103613",
        Expr::parse("acos(5/7)").expect("Failed to parse simple integer literal")
            .eval::<Number>().expect("Evaluation failed").to_string()
    );
    assert_eq!("0.6202494859828215",
        Expr::parse("atan(5/7)").expect("Failed to parse simple integer literal")
            .eval::<Number>().expect("Evaluation failed").to_string()
    );
    assert_eq!("0.6202494859828215",
        Expr::parse("atan2(5, 7)").expect("Failed to parse simple integer literal")
            .eval::<Number>().expect("Evaluation failed").to_string()
    );
    assert_eq!("0.7765927053545946",
        Expr::parse("sinh(5/7)").expect("Failed to parse simple integer literal")
            .eval::<Number>().expect("Evaluation failed").to_string()
    );
    assert_eq!("1.2661343649115475",
        Expr::parse("cosh(5/7)").expect("Failed to parse simple integer literal")
            .eval::<Number>().expect("Evaluation failed").to_string()
    );
    assert_eq!("0.6133572603953827",
        Expr::parse("tanh(5/7)").expect("Failed to parse simple integer literal")
            .eval::<Number>().expect("Evaluation failed").to_string()
    );
    assert_eq!("0.6643306045898852",
        Expr::parse("asinh(5/7)").expect("Failed to parse simple integer literal")
            .eval::<Number>().expect("Evaluation failed").to_string()
    );
    assert_eq!("0.8955880995299759",
        Expr::parse("acosh(10/7)").expect("Failed to parse simple integer literal")
            .eval::<Number>().expect("Evaluation failed").to_string()
    );
    assert_eq!("0.8958797346140275",
        Expr::parse("atanh(5/7)").expect("Failed to parse simple integer literal")
            .eval::<Number>().expect("Evaluation failed").to_string()
    );
}

#[test]
fn function_min() {
    assert_eq!("3/4",
        Expr::parse("min(20/7, 12/4, 9/12)").expect("Failed to parse simple integer literal")
            .eval::<Number>().expect("Evaluation failed").to_string()
    );
    assert_eq!("-10/7",
        Expr::parse("min(20/7, 12/4, 9/12, -3/61, -10/7)").expect("Failed to parse simple integer literal")
            .eval::<Number>().expect("Evaluation failed").to_string()
    );
    assert!(Expr::parse("min()").expect("Failed to parse simple integer literal").eval::<Number>().is_err());
}

#[test]
fn function_max() {
    assert_eq!("3",
        Expr::parse("max(20/7, 12/4, 9/12)").expect("Failed to parse simple integer literal")
            .eval::<Number>().expect("Evaluation failed").to_string()
    );
    assert_eq!("3",
        Expr::parse("max(20/7, 12/4, 9/12, -3/61, -10/7)").expect("Failed to parse simple integer literal")
            .eval::<Number>().expect("Evaluation failed").to_string()
    );
    assert!(Expr::parse("max()").expect("Failed to parse simple integer literal").eval::<Number>().is_err());
}

#[test]
fn constants() {
    let parsed = Expr::parse("e")
        .expect("Failed to parse simple integer literal");
    assert_eq!("2.718281828459045", parsed.eval::<Number>().expect("Evaluation failed").to_string());
    let parsed = Expr::parse("pi")
        .expect("Failed to parse simple integer literal");
    assert_eq!("3.141592653589793", parsed.eval::<Number>().expect("Evaluation failed").to_string());
    let parsed = Expr::parse("e ^ 2")
        .expect("Failed to parse simple integer literal");
    assert_eq!("7.3890560989306495", parsed.eval::<Number>().expect("Evaluation failed").to_string());
    let parsed = Expr::parse("pi ^ 2")
        .expect("Failed to parse simple integer literal");
    assert_eq!("9.869604401089358", parsed.eval::<Number>().expect("Evaluation failed").to_string());
}

#[test]
fn context_variable() {
    let mut context = NumberContext::new();
    context.set_variable("x", Number::from_str("12").expect("Failed parsing number"));
    let parsed = Expr::parse("42x")
        .expect("Failed to parse simple integer literal");
    assert_eq!("504", parsed.eval_in(&context).expect("Evaluation failed").to_string());
}

#[test]
fn context_function() {
    let mut context = NumberContext::new();
    context.set_function("x", Box::new(|v|
        v[0].clone().add(Number::from_str("12").expect("Failed parsing number"))
    ));
    let parsed = Expr::parse("x(42)")
        .expect("Failed to parse simple integer literal");
    assert_eq!("54", parsed.eval_in(&context).expect("Evaluation failed").to_string());
}

