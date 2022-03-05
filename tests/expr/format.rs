
use matheval::Expr;

#[test]
fn literal() {
    assert_eq!("1234", Expr::Literal("1234".to_owned()).to_string());
}

#[test]
fn variable() {
    assert_eq!("__abc1234", Expr::Variable("__abc1234".to_owned()).to_string());
}

#[test]
fn neg() {
    assert_eq!("-a1", Expr::Neg(Box::new(Expr::Variable("a1".to_owned()))).to_string());
}

#[test]
fn add() {
    assert_eq!("a1 + a2", Expr::Add(
        Box::new(Expr::Variable("a1".to_owned())),
        Box::new(Expr::Variable("a2".to_owned()))
    ).to_string());
}

#[test]
fn sub() {
    assert_eq!("a1 - a2", Expr::Sub(
        Box::new(Expr::Variable("a1".to_owned())),
        Box::new(Expr::Variable("a2".to_owned()))
    ).to_string());
}

#[test]
fn mul() {
    assert_eq!("a1 * a2", Expr::Mul(
        Box::new(Expr::Variable("a1".to_owned())),
        Box::new(Expr::Variable("a2".to_owned()))
    ).to_string());
}

#[test]
fn div() {
    assert_eq!("a1 / a2", Expr::Div(
        Box::new(Expr::Variable("a1".to_owned())),
        Box::new(Expr::Variable("a2".to_owned()))
    ).to_string());
}

#[test]
fn pow() {
    assert_eq!("a1^a2", Expr::Pow(
        Box::new(Expr::Variable("a1".to_owned())),
        Box::new(Expr::Variable("a2".to_owned()))
    ).to_string());
}

#[test]
fn func() {
    assert_eq!("func(a1, a2)", Expr::Function("func".to_owned(), vec![
        Expr::Variable("a1".to_owned()),
        Expr::Variable("a2".to_owned())
    ]).to_string());
}

#[test]
fn func_arg_exprs() {
    assert_eq!("func(a1 + a2 * a3, a4 - a5)", Expr::Function("func".to_owned(), vec![
        Expr::Add(
            Box::new(Expr::Variable("a1".to_owned())),
            Box::new(Expr::Mul(
                Box::new(Expr::Variable("a2".to_owned())),
                Box::new(Expr::Variable("a3".to_owned()))
            ))
        ),
        Expr::Sub(
            Box::new(Expr::Variable("a4".to_owned())),
            Box::new(Expr::Variable("a5".to_owned()))
        )
    ]).to_string());
}

#[test]
fn add_parens() {
    assert_eq!("a1 + (a2 + a3)", Expr::Add(
        Box::new(Expr::Variable("a1".to_owned())),
        Box::new(Expr::Add(
            Box::new(Expr::Variable("a2".to_owned())),
            Box::new(Expr::Variable("a3".to_owned()))
        ))
    ).to_string());
}

#[test]
fn add_sub_no_parens() {
    assert_eq!("a1 + a2 - a3", Expr::Sub(
        Box::new(Expr::Add(
            Box::new(Expr::Variable("a1".to_owned())),
            Box::new(Expr::Variable("a2".to_owned()))
        )),
        Box::new(Expr::Variable("a3".to_owned()))
    ).to_string());
}

#[test]
fn add_mul_parens() {
    assert_eq!("(a1 + a2) * a3", Expr::Mul(
        Box::new(Expr::Add(
            Box::new(Expr::Variable("a1".to_owned())),
            Box::new(Expr::Variable("a2".to_owned()))
        )),
        Box::new(Expr::Variable("a3".to_owned()))
    ).to_string());
    assert_eq!("a3 * (a1 + a2)", Expr::Mul(
        Box::new(Expr::Variable("a3".to_owned())),
        Box::new(Expr::Add(
            Box::new(Expr::Variable("a1".to_owned())),
            Box::new(Expr::Variable("a2".to_owned()))
        ))
    ).to_string());
}

#[test]
fn add_mul_no_parens() {
    assert_eq!("a1 * a2 + a3", Expr::Add(
        Box::new(Expr::Mul(
            Box::new(Expr::Variable("a1".to_owned())),
            Box::new(Expr::Variable("a2".to_owned()))
        )),
        Box::new(Expr::Variable("a3".to_owned()))
    ).to_string());
    assert_eq!("a3 + a1 * a2", Expr::Add(
        Box::new(Expr::Variable("a3".to_owned())),
        Box::new(Expr::Mul(
            Box::new(Expr::Variable("a1".to_owned())),
            Box::new(Expr::Variable("a2".to_owned()))
        ))
    ).to_string());
}

#[test]
fn sub_div_parens() {
    assert_eq!("(a1 - a2) / a3", Expr::Div(
        Box::new(Expr::Sub(
            Box::new(Expr::Variable("a1".to_owned())),
            Box::new(Expr::Variable("a2".to_owned()))
        )),
        Box::new(Expr::Variable("a3".to_owned()))
    ).to_string());
    assert_eq!("a3 / (a1 - a2)", Expr::Div(
        Box::new(Expr::Variable("a3".to_owned())),
        Box::new(Expr::Sub(
            Box::new(Expr::Variable("a1".to_owned())),
            Box::new(Expr::Variable("a2".to_owned()))
        ))
    ).to_string());
}

#[test]
fn mul_parens() {
    assert_eq!("a1 * (a2 * a3)", Expr::Mul(
        Box::new(Expr::Variable("a1".to_owned())),
        Box::new(Expr::Mul(
            Box::new(Expr::Variable("a2".to_owned())),
            Box::new(Expr::Variable("a3".to_owned()))
        ))
    ).to_string());
}

#[test]
fn mul_div_no_parens() {
    assert_eq!("a1 * a2 / a3", Expr::Div(
        Box::new(Expr::Mul(
            Box::new(Expr::Variable("a1".to_owned())),
            Box::new(Expr::Variable("a2".to_owned()))
        )),
        Box::new(Expr::Variable("a3".to_owned()))
    ).to_string());
}

#[test]
fn mul_pow_parens() {
    assert_eq!("(a1 * a2)^a3", Expr::Pow(
        Box::new(Expr::Mul(
            Box::new(Expr::Variable("a1".to_owned())),
            Box::new(Expr::Variable("a2".to_owned()))
        )),
        Box::new(Expr::Variable("a3".to_owned()))
    ).to_string());
    assert_eq!("a3^(a1 * a2)", Expr::Pow(
        Box::new(Expr::Variable("a3".to_owned())),
        Box::new(Expr::Mul(
            Box::new(Expr::Variable("a1".to_owned())),
            Box::new(Expr::Variable("a2".to_owned()))
        ))
    ).to_string());
}

#[test]
fn mul_pow_no_parens() {
    assert_eq!("a1^a2 * a3", Expr::Mul(
        Box::new(Expr::Pow(
            Box::new(Expr::Variable("a1".to_owned())),
            Box::new(Expr::Variable("a2".to_owned()))
        )),
        Box::new(Expr::Variable("a3".to_owned()))
    ).to_string());
    assert_eq!("a3 * a1^a2", Expr::Mul(
        Box::new(Expr::Variable("a3".to_owned())),
        Box::new(Expr::Pow(
            Box::new(Expr::Variable("a1".to_owned())),
            Box::new(Expr::Variable("a2".to_owned()))
        ))
    ).to_string());
}

#[test]
fn pow_parens() {
    assert_eq!("(a1^a2)^a3", Expr::Pow(
        Box::new(Expr::Pow(
            Box::new(Expr::Variable("a1".to_owned())),
            Box::new(Expr::Variable("a2".to_owned()))
        )),
        Box::new(Expr::Variable("a3".to_owned()))
    ).to_string());
}

#[test]
fn pow_no_parens() {
    assert_eq!("a1^a2^a3", Expr::Pow(
        Box::new(Expr::Variable("a1".to_owned())),
        Box::new(Expr::Pow(
            Box::new(Expr::Variable("a2".to_owned())),
            Box::new(Expr::Variable("a3".to_owned()))
        )),
    ).to_string());
}

