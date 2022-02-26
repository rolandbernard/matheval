
use super::Expr;

#[derive(Clone, Copy, PartialEq, Debug)]
enum TokenKind {
    Unknown,
    Identifier,
    Literal,
    Operator(char),
    Separator(char),
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
        return self.next.as_ref().and_then(|x| Some(x.kind));
    }

    fn find_next(&mut self) -> Option<Token> {
        let source = &self.source;
        let pos = &mut self.position;
        while *pos < source.len() && source[*pos].is_whitespace() {
            *pos += 1;
        }
        if *pos == source.len() {
            return None;
        } else {
            if source[*pos].is_digit(10) {
                let start = *pos;
                let base;
                if *pos + 2 < source.len() && source[*pos + 1] == 'b' && source[*pos + 2].is_digit(2) {
                    *pos += 2;
                    base = 2;
                } else if *pos + 2 < source.len() && source[*pos + 1] == 'o' && source[*pos + 2].is_digit(8) {
                    *pos += 2;
                    base = 8;
                } else if *pos + 2 < source.len() && source[*pos + 1] == 'x' && source[*pos + 2].is_digit(16) {
                    *pos += 2;
                    base = 16;
                } else {
                    *pos += 1;
                    base = 10;
                }
                while *pos < source.len() && source[*pos].is_digit(base) {
                    *pos += 1;
                }
                if *pos + 1 < source.len() && source[*pos] == '.' && source[*pos + 1].is_digit(base) {
                    *pos += 2;
                    while *pos < source.len() && source[*pos].is_digit(base) {
                        *pos += 1;
                    }
                }
                if *pos < source.len() && source[*pos] == 'e' {
                    if *pos + 1 < source.len() && source[*pos + 1].is_digit(base) {
                        *pos += 2;
                    } else if *pos + 2 < source.len() && (source[*pos + 1] == '+' || source[*pos + 1] == '-') && source[*pos + 2].is_digit(base) {
                        *pos += 3;
                    }
                    while *pos < source.len() && source[*pos].is_digit(base) {
                        *pos += 1;
                    }
                }
                return Some(Token {
                    kind: TokenKind::Literal, position: start,
                    source: Some(source[start..*pos].iter().collect::<String>())
                });
            } else if source[*pos].is_alphabetic() || source[*pos] == '_' {
                let start = *pos;
                while *pos < source.len() && (source[*pos].is_alphanumeric() || source[*pos] == '_') {
                    *pos += 1;
                }
                return Some(Token {
                    kind: TokenKind::Identifier, position: start,
                    source: Some(source[start..*pos].iter().collect::<String>())
                });
            } else if let '(' | '[' | '{' =  source[*pos] {
                let c = source[*pos];
                *pos += 1;
                return Some(Token { kind: TokenKind::OpenBracket(c), position: *pos - 1, source: None });
            } else if let ')' | ']' | '}' =  source[*pos] {
                let c = source[*pos];
                *pos += 1;
                return Some(Token { kind: TokenKind::CloseBracket(c), position: *pos - 1, source: None });
            } else if let '+' | '-' | '*' | '/' | '^' =  source[*pos] {
                let c = source[*pos];
                *pos += 1;
                return Some(Token { kind: TokenKind::Operator(c), position: *pos - 1, source: None });
            } else if let ',' | ';' =  source[*pos] {
                let c = source[*pos];
                *pos += 1;
                return Some(Token { kind: TokenKind::Separator(c), position: *pos - 1, source: None });
            } else {
                *pos += 1;
                return Some(Token { kind: TokenKind::Unknown, position: *pos - 1, source: None });
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
    let mut tokens = ExprTokenizer::on(s);
    return parse_root(&mut tokens);
}

fn parse_root(tokens: &mut ExprTokenizer) -> Result<Expr, ParseError> {
    let expr = parse_expr(tokens)?;
    if let Some(t) = tokens.next() {
        return Err(ParseError::from(&t, "Expected the end of input"));
    } else {
        return Ok(expr);
    }
}

fn parse_expr(tokens: &mut ExprTokenizer) -> Result<Expr, ParseError> {
    parse_sum(tokens)
}

fn parse_sum(tokens: &mut ExprTokenizer) -> Result<Expr, ParseError> {
    let mut sum = parse_product(tokens)?;
    while let Some(TokenKind::Operator(c)) = tokens.peek_kind() {
        if c == '+' {
            tokens.next();
            sum = Expr::Add(Box::new(sum), Box::new(parse_product(tokens)?));
        } else if c == '-' {
            tokens.next();
            sum = Expr::Sub(Box::new(sum), Box::new(parse_product(tokens)?));
        } else {
            break;
        }
    }
    return Ok(sum);
}

fn parse_product(tokens: &mut ExprTokenizer) -> Result<Expr, ParseError> {
    let mut prod = parse_power(tokens)?;
    loop {
        let kind = tokens.peek_kind();
        if kind == Some(TokenKind::Operator('*')) {
            tokens.next();
            prod = Expr::Mul(Box::new(prod), Box::new(parse_power(tokens)?));
        } else if kind == Some(TokenKind::Operator('/')) {
            tokens.next();
            prod = Expr::Div(Box::new(prod), Box::new(parse_power(tokens)?));
        } else if let Some(TokenKind::Identifier | TokenKind::Literal | TokenKind::OpenBracket(_)) = kind {
            prod = Expr::Mul(Box::new(prod), Box::new(parse_power(tokens)?));
        } else {
            break;
        }
    }
    return Ok(prod);
}

fn parse_power(tokens: &mut ExprTokenizer) -> Result<Expr, ParseError> {
    let mut pow = parse_base(tokens)?;
    if let Some(TokenKind::Operator('^')) = tokens.peek_kind() {
        tokens.next();
        pow = Expr::Pow(Box::new(pow), Box::new(parse_power(tokens)?));
    }
    return Ok(pow);
}

fn parse_base(tokens: &mut ExprTokenizer) -> Result<Expr, ParseError> {
    if let Some(TokenKind::Operator('+')) = tokens.peek_kind() {
        tokens.next();
        return parse_base(tokens);
    } else if let Some(TokenKind::Operator('-')) = tokens.peek_kind() {
        tokens.next();
        return Ok(Expr::Neg(Box::new(parse_base(tokens)?)));
    } else if let Some(TokenKind::Identifier) = tokens.peek_kind() {
        let name = tokens.next().unwrap();
        if let Some(TokenKind::OpenBracket('(')) = tokens.peek_kind() {
            tokens.next();
            let mut args = Vec::new();
            args.push(parse_expr(tokens)?);
            while let Some(TokenKind::Separator(',')) = tokens.peek_kind() {
                tokens.next();
                args.push(parse_expr(tokens)?);
            }
            let closing = tokens.next();
            if let Some(Token { kind: TokenKind::CloseBracket(')'), .. }) = closing {
                return Ok(Expr::Function(name.source.unwrap(), args));
            } else {
                return Err(ParseError::from(
                    &closing.unwrap_or(tokens.empty()),
                    "Expected matching closing bracket for function arguments"
                ));
            }
        } else {
            return Ok(Expr::Variable(name.source.unwrap()));
        }
    } else if let Some(TokenKind::OpenBracket('(')) = tokens.peek_kind() {
        tokens.next();
        let expr = parse_expr(tokens)?;
        let closing = tokens.next();
        if let Some(Token { kind: TokenKind::CloseBracket(')'), .. }) = closing {
            return Ok(expr);
        } else {
            return Err(ParseError::from(&closing.unwrap_or(tokens.empty()), "Expected matching closing bracket"));
        }
    } else if let Some(TokenKind::Literal) = tokens.peek_kind() {
        return Ok(Expr::Literal(tokens.next().unwrap().source.unwrap()));
    } else {
        return Err(ParseError::from(&tokens.next().unwrap_or(tokens.empty()), "Expected an expression"));
    }
}

