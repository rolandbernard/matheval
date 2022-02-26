
use matheval::Expr;

#[test]
fn parse_simple_integer_literal() {
    let parsed = Expr::parse("42").expect("Failed to parse simple integer literal");
    assert_eq!(Expr::Literal("42".to_owned()), parsed);
}

#[test]
fn parse_long_integer_literal() {
    let parsed = Expr::parse("1606938044258990275541962092341162602522202993782792835301376")
        .expect("Failed to parse long integer literal");
    assert_eq!(Expr::Literal("1606938044258990275541962092341162602522202993782792835301376".to_owned()), parsed);
}

#[test]
fn parse_simple_binary_literal() {
    let parsed = Expr::parse("0b1010100011110")
        .expect("Failed to parse binary integer literal");
    assert_eq!(Expr::Literal("0b1010100011110".to_owned()), parsed);
}

#[test]
fn parse_simple_octal_literal() {
    let parsed = Expr::parse("0o123765400")
        .expect("Failed to parse octal integer literal");
    assert_eq!(Expr::Literal("0o123765400".to_owned()), parsed);
}

#[test]
fn parse_simple_hex_literal() {
    let parsed = Expr::parse("0x24680afbcde13579")
        .expect("Failed to parse hex integer literal");
    assert_eq!(Expr::Literal("0x24680afbcde13579".to_owned()), parsed);
}

