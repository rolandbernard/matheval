
#[derive(PartialEq, Debug)]
pub enum Expr {
    Literal(String),
    Neg(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
    Function(String, Vec<Expr>),
    Variable(String),
}

