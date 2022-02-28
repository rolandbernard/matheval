
use std::ops::Add;
use std::str::FromStr;

use matheval::Context;
use matheval::Expr;
use matheval::Number;
use matheval::NumberContext;

#[test]
fn eval_simple_integer_literal() {
    let parsed = Expr::parse("98765432109876543210")
        .expect("Failed to parse simple integer literal");
    assert_eq!(
        Number::from_str("98765432109876543210").expect("Failed parsing number"),
        parsed.eval().expect("Evaluation failed")
    );
}

#[test]
fn eval_binary_integer_literal() {
    let parsed = Expr::parse("0b1001")
        .expect("Failed to parse simple integer literal");
    assert_eq!(
        Number::from_str("9").expect("Failed parsing number"),
        parsed.eval().expect("Evaluation failed")
    );
}

#[test]
fn eval_octal_integer_literal() {
    let parsed = Expr::parse("0o12017")
        .expect("Failed to parse simple integer literal");
    assert_eq!(
        Number::from_str("5135").expect("Failed parsing number"),
        parsed.eval().expect("Evaluation failed")
    );
}

#[test]
fn eval_hex_integer_literal() {
    let parsed = Expr::parse("0xfa0c")
        .expect("Failed to parse simple integer literal");
    assert_eq!(
        Number::from_str("64012").expect("Failed parsing number"),
        parsed.eval().expect("Evaluation failed")
    );
}

#[test]
fn eval_nonint_literal() {
    let parsed = Expr::parse("12.56")
        .expect("Failed to parse simple integer literal");
    assert_eq!(
        Number::from_str("12.56").expect("Failed parsing number"),
        parsed.eval().expect("Evaluation failed")
    );
}

#[test]
fn eval_exp_literal() {
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
fn eval_simple_add() {
    let parsed = Expr::parse("5 + 6e-5 + 42.5")
        .expect("Failed to parse simple integer literal");
    assert_eq!(
        Number::from_str("47.50006").expect("Failed parsing number"),
        parsed.eval().expect("Evaluation failed")
    );
}

#[test]
fn eval_simple_sub() {
    let parsed = Expr::parse("5e2 - 6 - 42.5")
        .expect("Failed to parse simple integer literal");
    assert_eq!(
        Number::from_str("451.5").expect("Failed parsing number"),
        parsed.eval().expect("Evaluation failed")
    );
}

#[test]
fn eval_simple_mul() {
    let parsed = Expr::parse("5 * 6e-5 * 42.5")
        .expect("Failed to parse simple integer literal");
    assert_eq!(
        Number::from_str("0.01275").expect("Failed parsing number"),
        parsed.eval().expect("Evaluation failed")
    );
}

#[test]
fn eval_simple_div() {
    let parsed = Expr::parse("5e2 / 6 / 42.5")
        .expect("Failed to parse simple integer literal");
    assert_eq!("100/51", parsed.eval::<Number>().expect("Evaluation failed").to_string());
}

#[test]
fn eval_simple_pow() {
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
fn eval_constants() {
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
fn eval_context_variable() {
    let mut context = NumberContext::new();
    context.set_variable("x", Number::from_str("12").expect("Failed parsing number"));
    let parsed = Expr::parse("42x")
        .expect("Failed to parse simple integer literal");
    assert_eq!("504", parsed.eval_in(&context).expect("Evaluation failed").to_string());
}

#[test]
fn eval_context_function() {
    let mut context = NumberContext::new();
    context.set_function("x", Box::new(|v|
        v[0].clone().add(Number::from_str("12").expect("Failed parsing number"))
    ));
    let parsed = Expr::parse("x(42)")
        .expect("Failed to parse simple integer literal");
    assert_eq!("54", parsed.eval_in(&context).expect("Evaluation failed").to_string());
}

