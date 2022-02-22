
use super::{expr::Expr, value::Value};

#[derive(Clone, Copy, PartialEq, Debug)]
enum TokenKind {
    Unknown,
    Identifier,
    Literal,
    Operator(char),
    OpenBracket(char),
    CloseBracket(char),
}

#[derive(Clone, PartialEq, Debug)]
struct Token {
    kind: TokenKind,
    position: usize,
    source: Option<String>,
}

#[derive(Debug)]
struct ExprTokenizer {
    position: usize,
    source: Vec<char>,
    next: Option<Token>
}

impl Iterator for ExprTokenizer {
    type Item = Token;
    
    fn next(&mut self) -> Option<Token> {
        if let Some(t) = std::mem::replace(&mut self.next, None) {
            self.next = None;
            return Some(t);
        } else {
            return self.find_next();
        }
    }
}

impl ExprTokenizer {
    fn on(s: &str) -> ExprTokenizer {
        return ExprTokenizer {
            position: 0, source: s.chars().collect::<Vec<_>>(), next: None
        };
    }

    fn peek_kind(&mut self) -> Option<TokenKind> {
        if self.next == None {
            self.next = self.find_next();
        }
        return std::mem::replace(&mut self.next, None).and_then(|x| Some(x.kind));
    }

    fn find_next(&mut self) -> Option<Token> {
        while self.position < self.source.len() && self.source[self.position].is_whitespace() {
            self.position += 1;
        }
        if self.position == self.source.len() {
            return None;
        } else {
            if self.source[self.position].is_digit(10) {
                let start = self.position;
                let mut str = String::new();
                while self.position < self.source.len() && self.source[self.position].is_digit(10) {
                    str.push(self.source[self.position]);
                    self.position += 1;
                }
                if self.position < self.source.len() && self.source[self.position] == '.' {
                    self.position += 1;
                    while self.position < self.source.len() && self.source[self.position].is_digit(10) {
                        str.push(self.source[self.position]);
                        self.position += 1;
                    }
                }
                return Some(Token { kind: TokenKind::Literal, position: start, source: Some(str) });
            } else if self.source[self.position].is_alphabetic() {
                let start = self.position;
                let mut str = String::new();
                while self.position < self.source.len() && self.source[self.position].is_alphabetic() {
                    str.push(self.source[self.position]);
                    self.position += 1;
                }
                return Some(Token { kind: TokenKind::Identifier, position: start, source: Some(str) });
            } else if let '(' | '[' | '{' =  self.source[self.position] {
                let c = self.source[self.position];
                self.position += 1;
                return Some(Token { kind: TokenKind::OpenBracket(c), position: self.position - 1, source: None });
            } else if let ')' | ']' | '}' =  self.source[self.position] {
                let c = match self.source[self.position] {
                    ')' => '(', ']' => '[', '}' => '{', _ => panic!(),
                };
                self.position += 1;
                return Some(Token { kind: TokenKind::CloseBracket(c), position: self.position - 1, source: None });
            } else if let '+' | '-' | '*' | '/' | '^' =  self.source[self.position] {
                let c = self.source[self.position];
                self.position += 1;
                return Some(Token { kind: TokenKind::Operator(c), position: self.position - 1, source: None });
            } else {
                self.position += 1;
                return Some(Token { kind: TokenKind::Unknown, position: self.position - 1, source: None });
            }
        }
    }
    
    fn empty(&self) -> Token {
        let position;
        if let Some(Token { position: pos , .. }) = self.next {
            position = pos;
        } else {
            position = self.position;
        }
        return Token { kind: TokenKind::Unknown, position, source: None };
    }
}

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub position: usize,
}

impl ParseError {
    fn from(tok: &Token, msg: &str) -> ParseError {
        return ParseError { message: msg.to_owned(), position: tok.position };
    }
}

pub fn parse(s: &str) -> Result<Expr, ParseError> {
    let tokens = ExprTokenizer::on(s);
    println!("{:?}", tokens.collect::<Vec<_>>());
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
            return Ok(Expr::Function(name.source.unwrap(), Box::new(parse_bracketed(tokens)?)));
        } else {
            return Ok(Expr::Variable(name.source.unwrap()));
        }
    } else if let Some(TokenKind::OpenBracket(_)) = tokens.peek_kind() {
        return parse_bracketed(tokens);
    } else if let Some(TokenKind::Literal) = tokens.peek_kind() {
        let mut num = 0;
        let mut den = 1;
        let mut dec = false;
        for c in tokens.next().unwrap().source.unwrap().chars() {
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

