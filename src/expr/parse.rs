
use super::{Expr, value::Value};

#[derive(Clone, Copy, PartialEq)]
enum TokenKind {
    Unknown,
    Identifier,
    Integer,
    Float,
    Operator(char),
    OpenBracket(char),
    CloseBracket(char),
}

#[derive(Clone, Copy, PartialEq)]
struct Token<'a> {
    kind: TokenKind,
    position: usize,
    source: &'a str,
}

struct ExprTokenizer<'a> {
    position: usize,
    source: &'a str,
    next: Option<Token<'a>>
}

impl<'a> ExprTokenizer<'a> {
    fn on(s: &'a str) -> ExprTokenizer<'a> {
        return ExprTokenizer {
            position: 0, source: s, next: None
        };
    }

    fn next(&mut self) -> Option<Token<'a>> {
        if let Some(t) = self.next {
            self.next = None;
            return Some(t);
        } else {
            return self.find_next();
        }
    }

    fn peek(&mut self) -> Option<Token<'a>> {
        if self.next == None {
            self.next = self.find_next();
        }
        return self.next;
    }

    fn peek_kind(&mut self) -> Option<TokenKind> {
        return self.peek().and_then(|x| Some(x.kind));
    }

    fn find_next(&mut self) -> Option<Token<'a>> {
        todo!();
    }
    
    fn empty(&self) -> Token<'static> {
        let position;
        if let Some(Token { position: pos , .. }) = self.next {
            position = pos;
        } else {
            position = self.position;
        }
        return Token { kind: TokenKind::Unknown, position, source: "" };
    }
}

pub struct ParseError {
    message: String,
    position: usize,
}

impl ParseError {
    fn from(tok: &Token, msg: &str) -> ParseError {
        return ParseError { message: msg.to_owned(), position: tok.position };
    }
}

pub fn parse(s: &str) -> Result<Expr, ParseError> {
    let mut tokens = ExprTokenizer::on(s);
    return parse_expr(&mut tokens);
}

fn parse_expr(tokens: &mut ExprTokenizer) -> Result<Expr, ParseError> {
    let expr = parse_sum(tokens)?;
    if let Some(t) = tokens.next() {
        return Err(ParseError::from(&t, "Expected the end of input"));
    } else {
        return Ok(expr);
    }
}

fn parse_sum(tokens: &mut ExprTokenizer) -> Result<Expr, ParseError> {
    let mut all = Vec::new();
    all.push(parse_product(tokens)?);
    while let Some(TokenKind::Operator(c)) = tokens.peek_kind() {
        if c == '+' {
            tokens.next();
            all.push(parse_product(tokens)?);
        } else if c == '-' {
            tokens.next();
            all.push(Expr::negate(parse_product(tokens)?));
        } else {
            break;
        }
    }
    if all.len() == 1 {
        return Ok(all.pop().unwrap());
    } else {
        return Ok(Expr::Sum(all));
    }
}

fn parse_product(tokens: &mut ExprTokenizer) -> Result<Expr, ParseError> {
    let mut all = Vec::new();
    all.push(parse_power(tokens)?);
    while let Some(TokenKind::Operator(c)) = tokens.peek_kind() {
        if c == '*' {
            tokens.next();
            all.push(parse_power(tokens)?);
        } else if c == '/' {
            tokens.next();
            all.push(Expr::reciprocal(parse_power(tokens)?));
        } else {
            break;
        }
    }
    if all.len() == 1 {
        return Ok(all.pop().unwrap());
    } else {
        return Ok(Expr::Sum(all));
    }
}

fn parse_power(tokens: &mut ExprTokenizer) -> Result<Expr, ParseError> {
    let mut all = Vec::new();
    all.push(parse_base(tokens)?);
    while let Some(TokenKind::Operator('^')) = tokens.peek_kind() {
        tokens.next();
        all.push(parse_base(tokens)?);
    }
    if all.len() == 1 {
        return Ok(all.pop().unwrap());
    } else {
        return Ok(Expr::Sum(all));
    }
}

fn parse_base(tokens: &mut ExprTokenizer) -> Result<Expr, ParseError> {
    if let Some(TokenKind::Operator('-')) = tokens.peek_kind() {
        tokens.next();
        return Ok(Expr::negate(parse_base(tokens)?));
    } else if let Some(TokenKind::Identifier) = tokens.peek_kind() {
        let name = tokens.next().unwrap();
        if let Some(TokenKind::OpenBracket(_)) = tokens.peek_kind() {
            return Ok(Expr::Function(name.source.to_owned(), Box::new(parse_bracketed(tokens)?)));
        } else {
            return Ok(Expr::Variable(name.source.to_owned()));
        }
    } else if let Some(TokenKind::OpenBracket(_)) = tokens.peek_kind() {
        return parse_bracketed(tokens);
    } else if let Some(Token { kind: TokenKind::Float, source, .. }) = tokens.peek() {
        let mut num = 0;
        let mut den = 1;
        let mut dec = false;
        for c in source.chars() {
            if c == '.' {
                dec = true;
            } else {
                num *= 10;
                num += c.to_digit(10).unwrap() as i64;
                if dec {
                    den *= 10;
                }
            }
        }
        return Ok(Expr::Constant(Value::Rational(num, den)));
    } else if let Some(Token { kind: TokenKind::Integer, source, .. }) = tokens.peek() {
        let mut num = 0;
        for c in source.chars() {
            num *= 10;
            num += c.to_digit(10).unwrap() as i64;
        }
        return Ok(Expr::Constant(Value::Rational(num, 1)));
    } else {
        return Err(ParseError::from(&tokens.next().unwrap_or(tokens.empty()), "Expected an expression"));
    }
}

fn parse_bracketed(tokens: &mut ExprTokenizer) -> Result<Expr, ParseError> {
    if let Some(Token { kind: TokenKind::OpenBracket(o), .. }) = tokens.next() {
        let expr = parse_sum(tokens)?;
        let closing = tokens.next();
        if let Some(Token { kind: TokenKind::CloseBracket(c), .. }) = closing {
            if o == c {
                return Ok(expr);
            }
        }
        return Err(ParseError::from(&closing.unwrap_or(tokens.empty()), "Expected matching closing bracket"));
    } else {
        panic!();
    }
}

