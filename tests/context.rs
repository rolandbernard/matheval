
use std::str::FromStr;

use matheval::{NumberContext, Context, Number};

#[test]
fn context_has_pi() {
    let cnxt = NumberContext::new();
    let val = cnxt.get_variable("pi").expect("Context doesn't contain pi");
    assert_eq!(Number::Float(3.1415926535897932384626433832795), val);
}

#[test]
fn context_has_e() {
    let cnxt = NumberContext::new();
    let val = cnxt.get_variable("e").expect("Context doesn't contain e");
    assert_eq!(Number::Float(2.7182818284590452353602874713527), val);
}

#[test]
fn context_has_general_func() {
    let cnxt = NumberContext::new();
    cnxt.get_function("floor").expect("Context doesn't contain floor");
    cnxt.get_function("ceil").expect("Context doesn't contain ceil");
    cnxt.get_function("round").expect("Context doesn't contain round");
    cnxt.get_function("trunc").expect("Context doesn't contain trunc");
    cnxt.get_function("fract").expect("Context doesn't contain fract");
    cnxt.get_function("abs").expect("Context doesn't contain abs");
    cnxt.get_function("sign").expect("Context doesn't contain sign");
    cnxt.get_function("sqrt").expect("Context doesn't contain sqrt");
    cnxt.get_function("cbrt").expect("Context doesn't contain cbrt");
    cnxt.get_function("ln").expect("Context doesn't contain ln");
    cnxt.get_function("log").expect("Context doesn't contain log");
}

#[test]
fn context_has_trig_func() {
    let cnxt = NumberContext::new();
    cnxt.get_function("sin").expect("Context doesn't contain sin");
    cnxt.get_function("cos").expect("Context doesn't contain cos");
    cnxt.get_function("tan").expect("Context doesn't contain tan");
    cnxt.get_function("asin").expect("Context doesn't contain asin");
    cnxt.get_function("acos").expect("Context doesn't contain acos");
    cnxt.get_function("atan").expect("Context doesn't contain atan");
    cnxt.get_function("atan2").expect("Context doesn't contain ata2");
    cnxt.get_function("sinh").expect("Context doesn't contain sinh");
    cnxt.get_function("cosh").expect("Context doesn't contain cosh");
    cnxt.get_function("tanh").expect("Context doesn't contain tanh");
    cnxt.get_function("asinh").expect("Context doesn't contain asinh");
    cnxt.get_function("acosh").expect("Context doesn't contain acosh");
    cnxt.get_function("atanh").expect("Context doesn't contain atanh");
}

#[test]
fn context_set_variable() {
    let mut cnxt = NumberContext::new();
    let num = Number::from_str("12").expect("Failed parsing number");
    cnxt.set_variable("a1", num.clone());
    let val = cnxt.get_variable("a1").expect("Context doesn't contain a1");
    assert_eq!(num, val);
}

#[test]
fn context_overwrite_variable() {
    let mut cnxt = NumberContext::new();
    let num = Number::from_str("12").expect("Failed parsing number");
    cnxt.set_variable("a1", num.clone());
    let num = Number::from_str("42").expect("Failed parsing number");
    cnxt.set_variable("a1", num.clone());
    let val = cnxt.get_variable("a1").expect("Context doesn't contain a1");
    assert_eq!(num, val);
}

#[test]
fn context_overwrite_buildin_variable() {
    let mut cnxt = NumberContext::new();
    let num = Number::from_str("3.14").expect("Failed parsing number");
    cnxt.set_variable("pi", num.clone());
    let val = cnxt.get_variable("pi").expect("Context doesn't contain pi");
    assert_eq!(num, val);
}

#[test]
fn context_set_function() {
    let mut cnxt = NumberContext::new();
    let num = Number::from_str("42").expect("Failed parsing number");
    let func_num = num.clone();
    let func = move |_| Ok(func_num.clone());
    cnxt.set_function("func1", Box::new(func));
    let val = cnxt.get_function("func1")
        .expect("Context doesn't contain func1");
    assert_eq!(num, val(Vec::new()).expect("Function call failed"));
}

#[test]
fn context_overwrite_function() {
    let mut cnxt = NumberContext::new();
    let num = Number::from_str("12").expect("Failed parsing number");
    let func_num = num.clone();
    let func = move |_| Ok(func_num.clone());
    cnxt.set_function("func1", Box::new(func));
    let num = Number::from_str("42").expect("Failed parsing number");
    let func_num = num.clone();
    let func = move |_| Ok(func_num.clone());
    cnxt.set_function("func1", Box::new(func));
    let val = cnxt.get_function("func1")
        .expect("Context doesn't contain func1");
    assert_eq!(num, val(Vec::new()).expect("Function call failed"));
}

#[test]
fn context_overwrite_buildin_function() {
    let mut cnxt = NumberContext::new();
    let num = Number::from_str("42").expect("Failed parsing number");
    let func_num = num.clone();
    let func = move |_| Ok(func_num.clone());
    cnxt.set_function("sin", Box::new(func));
    let val = cnxt.get_function("sin")
        .expect("Context doesn't contain sin");
    assert_eq!(num, val(Vec::new()).expect("Function call failed"));
}

#[test]
fn context_unknown_variable() {
    let cnxt = NumberContext::new();
    assert_eq!(None, cnxt.get_variable("some_unknown_variable"));
}

#[test]
fn context_unknown_function() {
    let cnxt = NumberContext::new();
    assert!(cnxt.get_function("some_unknown_function").is_none());
}

