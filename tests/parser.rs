
use matheval::Expr;

#[test]
fn parse_simple_integer_literal() {
    let parsed = Expr::parse("98765432109876543210").expect("Failed to parse simple integer literal");
    assert_eq!(Expr::Literal("98765432109876543210".to_owned()), parsed);
}

#[test]
fn parse_long_integer_literal() {
    let parsed = Expr::parse("1606938044258990275541962092341162602522202993782792835301376")
        .expect("Failed to parse long integer literal");
    assert_eq!(Expr::Literal("1606938044258990275541962092341162602522202993782792835301376".to_owned()), parsed);
}

#[test]
fn parse_binary_integer_literal() {
    let parsed = Expr::parse("0b1010")
        .expect("Failed to parse binary integer literal");
    assert_eq!(Expr::Literal("0b1010".to_owned()), parsed);
}

#[test]
fn parse_octal_integer_literal() {
    let parsed = Expr::parse("0o7654321076543210")
        .expect("Failed to parse octal integer literal");
    assert_eq!(Expr::Literal("0o7654321076543210".to_owned()), parsed);
}

#[test]
fn parse_hex_integer_literal() {
    let parsed = Expr::parse("0xfedcba9876543210fedcba9876543210")
        .expect("Failed to parse hex integer literal");
    assert_eq!(Expr::Literal("0xfedcba9876543210fedcba9876543210".to_owned()), parsed);
}

#[test]
fn parse_simple_nonint_literal() {
    let parsed = Expr::parse("9876543210.0123456789")
        .expect("Failed to parse simple non-integer literal");
    assert_eq!(Expr::Literal("9876543210.0123456789".to_owned()), parsed);
    assert!(Expr::parse("98765432109876543210.").is_err());
    assert!(Expr::parse(".98765432109876543210").is_err());
}

#[test]
fn parse_binary_nonint_literal() {
    let parsed = Expr::parse("0b1010.0101")
        .expect("Failed to parse binary non-integer literal");
    assert_eq!(Expr::Literal("0b1010.0101".to_owned()), parsed);
    assert!(Expr::parse("0b1010.").is_err());
}

#[test]
fn parse_octal_nonint_literal() {
    let parsed = Expr::parse("0o76543210.01234567")
        .expect("Failed to parse octal non-integer literal");
    assert_eq!(Expr::Literal("0o76543210.01234567".to_owned()), parsed);
    assert!(Expr::parse("0o76543210.").is_err());
}

#[test]
fn parse_hex_nonint_literal() {
    let parsed = Expr::parse("0xfedcba9876543210.0123456789abcdef")
        .expect("Failed to parse hex non-integer literal");
    assert_eq!(Expr::Literal("0xfedcba9876543210.0123456789abcdef".to_owned()), parsed);
    assert!(Expr::parse("0xfedcba9876543210.").is_err());
}

#[test]
fn parse_simple_exponent_literal() {
    let parsed = Expr::parse("9876543210.0123456789e-0123456789")
        .expect("Failed to parse simple non-integer literal");
    assert_eq!(Expr::Literal("9876543210.0123456789e-0123456789".to_owned()), parsed);
    let parsed = Expr::parse("9876543210.0123456789e+0123456789")
        .expect("Failed to parse simple non-integer literal");
    assert_eq!(Expr::Literal("9876543210.0123456789e+0123456789".to_owned()), parsed);
    let parsed = Expr::parse("9876543210.0123456789e0123456789")
        .expect("Failed to parse simple non-integer literal");
    assert_eq!(Expr::Literal("9876543210.0123456789e0123456789".to_owned()), parsed);
    let parsed = Expr::parse("98765432100123456789e-0123456789")
        .expect("Failed to parse simple non-integer literal");
    assert_eq!(Expr::Literal("98765432100123456789e-0123456789".to_owned()), parsed);
    let parsed = Expr::parse("98765432100123456789e+0123456789")
        .expect("Failed to parse simple non-integer literal");
    assert_eq!(Expr::Literal("98765432100123456789e+0123456789".to_owned()), parsed);
    let parsed = Expr::parse("98765432100123456789e0123456789")
        .expect("Failed to parse simple non-integer literal");
    assert_eq!(Expr::Literal("98765432100123456789e0123456789".to_owned()), parsed);
}

#[test]
fn parse_variable() {
    let parsed = Expr::parse("abcxyz").expect("Failed to parse variable");
    assert_eq!(Expr::Variable("abcxyz".to_owned()), parsed);
    let parsed = Expr::parse("__abc__xyz").expect("Failed to parse variable");
    assert_eq!(Expr::Variable("__abc__xyz".to_owned()), parsed);
    let parsed = Expr::parse("abc123890").expect("Failed to parse variable");
    assert_eq!(Expr::Variable("abc123890".to_owned()), parsed);
    let parsed = Expr::parse("abc123xyz").expect("Failed to parse variable");
    assert_eq!(Expr::Variable("abc123xyz".to_owned()), parsed);
}

#[test]
fn parse_simple_add() {
    let parsed = Expr::parse("a1 + a2").expect("Failed to parse simple add");
    assert_eq!(Expr::Add(
        Box::new(Expr::Variable("a1".to_owned())),
        Box::new(Expr::Variable("a2".to_owned()))
    ), parsed);
}

#[test]
fn parse_simple_sub() {
    let parsed = Expr::parse("a1 - a2").expect("Failed to parse simple sub");
    assert_eq!(Expr::Sub(
        Box::new(Expr::Variable("a1".to_owned())),
        Box::new(Expr::Variable("a2".to_owned()))
    ), parsed);
}

#[test]
fn parse_mixed_add_sub() {
    let parsed = Expr::parse("a1 + a2 - a3 + a4").expect("Failed to parse mixed add/sub");
    assert_eq!(Expr::Add(
        Box::new(Expr::Sub(
            Box::new(Expr::Add(
                Box::new(Expr::Variable("a1".to_owned())),
                Box::new(Expr::Variable("a2".to_owned()))
            )),
            Box::new(Expr::Variable("a3".to_owned()))
        )),
        Box::new(Expr::Variable("a4".to_owned()))
    ), parsed);
}

#[test]
fn parse_simple_mul() {
    let parsed = Expr::parse("a1 * a2").expect("Failed to parse simple mul");
    assert_eq!(Expr::Mul(
        Box::new(Expr::Variable("a1".to_owned())),
        Box::new(Expr::Variable("a2".to_owned()))
    ), parsed);
}

#[test]
fn parse_implicit_mul() {
    let parsed = Expr::parse("12a1").expect("Failed to parse implicit mul");
    assert_eq!(Expr::Mul(
        Box::new(Expr::Literal("12".to_owned())),
        Box::new(Expr::Variable("a1".to_owned()))
    ), parsed);
    let parsed = Expr::parse("12 42").expect("Failed to parse implicit mul");
    assert_eq!(Expr::Mul(
        Box::new(Expr::Literal("12".to_owned())),
        Box::new(Expr::Literal("42".to_owned()))
    ), parsed);
    let parsed = Expr::parse("12(a1 + a2)").expect("Failed to parse implicit mul");
    assert_eq!(Expr::Mul(
        Box::new(Expr::Literal("12".to_owned())),
        Box::new(Expr::Add(
            Box::new(Expr::Variable("a1".to_owned())),
            Box::new(Expr::Variable("a2".to_owned()))
        ))
    ), parsed);
}

#[test]
fn parse_simple_div() {
    let parsed = Expr::parse("a1 / a2").expect("Failed to parse simple div");
    assert_eq!(Expr::Div(
        Box::new(Expr::Variable("a1".to_owned())),
        Box::new(Expr::Variable("a2".to_owned()))
    ), parsed);
}

#[test]
fn parse_mixed_mul_div() {
    let parsed = Expr::parse("a1 * a2 / a3 * a4").expect("Failed to parse mixed mul/div");
    assert_eq!(Expr::Mul(
        Box::new(Expr::Div(
            Box::new(Expr::Mul(
                Box::new(Expr::Variable("a1".to_owned())),
                Box::new(Expr::Variable("a2".to_owned()))
            )),
            Box::new(Expr::Variable("a3".to_owned()))
        )),
        Box::new(Expr::Variable("a4".to_owned()))
    ), parsed);
}

#[test]
fn parse_simple_pow() {
    let parsed = Expr::parse("a1 ^ a2").expect("Failed to parse simple pow");
    assert_eq!(Expr::Pow(
        Box::new(Expr::Variable("a1".to_owned())),
        Box::new(Expr::Variable("a2".to_owned()))
    ), parsed);
}

#[test]
fn parse_mixed_pow() {
    let parsed = Expr::parse("a1 ^ a2 ^ a3 ^ a4").expect("Failed to parse mixed pow");
    assert_eq!(Expr::Pow(
        Box::new(Expr::Variable("a1".to_owned())),
        Box::new(Expr::Pow(
            Box::new(Expr::Variable("a2".to_owned())),
            Box::new(Expr::Pow(
                Box::new(Expr::Variable("a3".to_owned())),
                Box::new(Expr::Variable("a4".to_owned()))
            ))
        ))
    ), parsed);
}

#[test]
fn parse_simple_neg() {
    let parsed = Expr::parse("--a1").expect("Failed to parse simple neg");
    assert_eq!(Expr::Neg(Box::new(Expr::Neg(
        Box::new(Expr::Variable("a1".to_owned()))
    ))), parsed);
}

#[test]
fn parse_simple_function() {
    let parsed = Expr::parse("func(a1)").expect("Failed to parse simple function");
    assert_eq!(Expr::Function("func".to_owned(), vec![Expr::Variable("a1".to_owned())]), parsed);
}

#[test]
fn parse_multi_variable_function() {
    let parsed = Expr::parse("func(a1, a2, a3, a4, a5)")
        .expect("Failed to parse multi-variable function");
    assert_eq!(Expr::Function("func".to_owned(), vec![
        Expr::Variable("a1".to_owned()),
        Expr::Variable("a2".to_owned()),
        Expr::Variable("a3".to_owned()),
        Expr::Variable("a4".to_owned()),
        Expr::Variable("a5".to_owned())
    ]), parsed);
}

#[test]
fn parse_nested_function() {
    let parsed = Expr::parse("func1(a1, func2(func3(a2), func4(a3, a4), a5))")
        .expect("Failed to parse nested function");
    assert_eq!(Expr::Function("func1".to_owned(), vec![
        Expr::Variable("a1".to_owned()),
        Expr::Function("func2".to_owned(), vec![
            Expr::Function("func3".to_owned(), vec![
                Expr::Variable("a2".to_owned())
            ]),
            Expr::Function("func4".to_owned(), vec![
                Expr::Variable("a3".to_owned()),
                Expr::Variable("a4".to_owned()),
            ]),
            Expr::Variable("a5".to_owned())
        ])
    ]), parsed);
}

#[test]
fn parse_precedence_add_mul() {
    let parsed = Expr::parse("a1 * a2 + a3 / a4 - a5 * a6 + a7 / a8")
        .expect("Failed to parse expression");
    assert_eq!(Expr::Add(
        Box::new(Expr::Sub(
            Box::new(Expr::Add(
                Box::new(Expr::Mul(
                    Box::new(Expr::Variable("a1".to_owned())),
                    Box::new(Expr::Variable("a2".to_owned()))
                )),
                Box::new(Expr::Div(
                    Box::new(Expr::Variable("a3".to_owned())),
                    Box::new(Expr::Variable("a4".to_owned()))
                ))
            )),
            Box::new(Expr::Mul(
                Box::new(Expr::Variable("a5".to_owned())),
                Box::new(Expr::Variable("a6".to_owned()))
            ))
        )),
        Box::new(Expr::Div(
            Box::new(Expr::Variable("a7".to_owned())),
            Box::new(Expr::Variable("a8".to_owned()))
        ))
    ), parsed);
}

#[test]
fn parse_precedence_mul_pow() {
    let parsed = Expr::parse("a1 ^ a2 * a3 ^ a4 / a5 ^ a6 * a7 ^ a8")
        .expect("Failed to parse expression");
    assert_eq!(Expr::Mul(
        Box::new(Expr::Div(
            Box::new(Expr::Mul(
                Box::new(Expr::Pow(
                    Box::new(Expr::Variable("a1".to_owned())),
                    Box::new(Expr::Variable("a2".to_owned()))
                )),
                Box::new(Expr::Pow(
                    Box::new(Expr::Variable("a3".to_owned())),
                    Box::new(Expr::Variable("a4".to_owned()))
                ))
            )),
            Box::new(Expr::Pow(
                Box::new(Expr::Variable("a5".to_owned())),
                Box::new(Expr::Variable("a6".to_owned()))
            ))
        )),
        Box::new(Expr::Pow(
            Box::new(Expr::Variable("a7".to_owned())),
            Box::new(Expr::Variable("a8".to_owned()))
        ))
    ), parsed);
}

#[test]
fn parse_precedence_neg_pow() {
    let parsed = Expr::parse("- a1 ^ - a2 ^ a3 ^ - a4")
        .expect("Failed to parse expression");
    assert_eq!(Expr::Pow(
        Box::new(Expr::Neg(Box::new(Expr::Variable("a1".to_owned())))),
        Box::new(Expr::Pow(
            Box::new(Expr::Neg(Box::new(Expr::Variable("a2".to_owned())))),
            Box::new(Expr::Pow(
                Box::new(Expr::Variable("a3".to_owned())),
                Box::new(Expr::Neg(Box::new(Expr::Variable("a4".to_owned()))))
            )),
        )),
    ), parsed);
}

#[test]
fn parse_precedence_parens() {
    let parsed = Expr::parse("a1 * (((a2 + a3) / (a4 - a5) * (a6 + a7)) / a8)")
        .expect("Failed to parse expression");
    assert_eq!(Expr::Mul(
        Box::new(Expr::Variable("a1".to_owned())),
        Box::new(Expr::Div(
            Box::new(Expr::Mul(
                Box::new(Expr::Div(
                    Box::new(Expr::Add(
                        Box::new(Expr::Variable("a2".to_owned())),
                        Box::new(Expr::Variable("a3".to_owned()))
                    )),
                    Box::new(Expr::Sub(
                        Box::new(Expr::Variable("a4".to_owned())),
                        Box::new(Expr::Variable("a5".to_owned()))
                    ))
                )),
                Box::new(Expr::Add(
                    Box::new(Expr::Variable("a6".to_owned())),
                    Box::new(Expr::Variable("a7".to_owned()))
                )),
            )),
            Box::new(Expr::Variable("a8".to_owned()))
        ))
    ), parsed);
}

#[test]
fn parse_precedence_add_pow() {
    let parsed = Expr::parse("a1 ^ a2 + a3 ^ a4 - a5 ^ a6 + a7 ^ a8")
        .expect("Failed to parse expression");
    assert_eq!(Expr::Add(
        Box::new(Expr::Sub(
            Box::new(Expr::Add(
                Box::new(Expr::Pow(
                    Box::new(Expr::Variable("a1".to_owned())),
                    Box::new(Expr::Variable("a2".to_owned()))
                )),
                Box::new(Expr::Pow(
                    Box::new(Expr::Variable("a3".to_owned())),
                    Box::new(Expr::Variable("a4".to_owned()))
                ))
            )),
            Box::new(Expr::Pow(
                Box::new(Expr::Variable("a5".to_owned())),
                Box::new(Expr::Variable("a6".to_owned()))
            ))
        )),
        Box::new(Expr::Pow(
            Box::new(Expr::Variable("a7".to_owned())),
            Box::new(Expr::Variable("a8".to_owned()))
        ))
    ), parsed);
}

#[test]
fn parse_precedence_add_mul_pow() {
    let parsed = Expr::parse("a1 ^ a2 - a3 ^ a4 / a5 ^ a6 * a7 ^ a8")
        .expect("Failed to parse expression");
    assert_eq!(Expr::Sub(
        Box::new(Expr::Pow(
            Box::new(Expr::Variable("a1".to_owned())),
            Box::new(Expr::Variable("a2".to_owned()))
        )),
        Box::new(Expr::Mul(
            Box::new(Expr::Div(
                Box::new(Expr::Pow(
                    Box::new(Expr::Variable("a3".to_owned())),
                    Box::new(Expr::Variable("a4".to_owned()))
                )),
                Box::new(Expr::Pow(
                    Box::new(Expr::Variable("a5".to_owned())),
                    Box::new(Expr::Variable("a6".to_owned()))
                ))
            )),
            Box::new(Expr::Pow(
                Box::new(Expr::Variable("a7".to_owned())),
                Box::new(Expr::Variable("a8".to_owned()))
            ))
        )),
    ), parsed);
}

#[test]
fn parse_error() {
    assert!(Expr::parse("").is_err(), "Empty expressions are illegal");
    assert!(Expr::parse("+").is_err(), "Expected value after unary '+'");
    assert!(Expr::parse("-").is_err(), "Expected value after unary '-'");
    assert!(Expr::parse("* a").is_err(), "Not a unary operator '*'");
    assert!(Expr::parse("/ a").is_err(), "Not a unary operator '/'");
    assert!(Expr::parse("^ a").is_err(), "Not a unary operator '^'");
    assert!(Expr::parse("a + ").is_err(), "Missing second '+' operand");
    assert!(Expr::parse("a - ").is_err(), "Missing second '-' operand");
    assert!(Expr::parse("a * ").is_err(), "Missing second '*' operand");
    assert!(Expr::parse("a / ").is_err(), "Missing second '/' operand");
    assert!(Expr::parse("a ^ ").is_err(), "Missing second '^' operand");
    assert!(Expr::parse("a +$ b").is_err(), "Unexpected character '$'");
    assert!(Expr::parse("a + b) * 2").is_err(), "Unexpected closing paren ')'");
    assert!(Expr::parse("(a + b * 2").is_err(), "Missing closing paren ')'");
    assert!(Expr::parse("sin(a + b * 2").is_err(), "Missing closing paren ')' for function");
    assert!(Expr::parse("sin(a + b, 2").is_err(), "Missing closing paren ')' for function");
    assert!(Expr::parse("sin(a + b, 2,)").is_err(), "Unexpected character ',' in function arguments");
    assert!(Expr::parse("sin(a + b, , 2)").is_err(), "Empty function argument");
    assert!(Expr::parse("a + () * b").is_err(), "Empty parens");
    assert!(Expr::parse("a + (b * (c *) 2)").is_err(), "Misplaced parens");
}

