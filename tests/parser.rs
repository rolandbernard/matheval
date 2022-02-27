
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




