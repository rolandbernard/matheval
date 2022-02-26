
use super::Expr;

pub fn format(expr: &Expr) -> String {
    format_prec(expr, 0)
}

fn expr_prec(expr: &Expr) -> usize {
    match expr {
        Expr::Add(..) => 10,
        Expr::Sub(..) => 10,
        Expr::Mul(..) => 20,
        Expr::Div(..) => 20,
        Expr::Pow(..) => 40,
        Expr::Neg(..) => 50,
        Expr::Function(..) => 50,
        Expr::Variable(..) => 50,
        Expr::Literal(..) => 50,
    }
}

fn format_prec(expr: &Expr, parent: usize) -> String {
    let prec = expr_prec(expr);
    let mut res = String::new();
    if parent > prec {
        res.push('(');
    }
    match expr {
        Expr::Add(l, r) => {
            res.push_str(&format_prec(l, prec));
            res.push_str(" + ");
            res.push_str(&format_prec(r, prec + 1));
        },
        Expr::Sub(l, r) => {
            res.push_str(&format_prec(l, prec));
            res.push_str(" - ");
            res.push_str(&format_prec(r, prec + 1));
        },
        Expr::Mul(l, r) => {
            res.push_str(&format_prec(l, prec));
            res.push_str(" * ");
            res.push_str(&format_prec(r, prec + 1));
        },
        Expr::Div(l, r) => {
            res.push_str(&format_prec(l, prec));
            res.push_str(" / ");
            res.push_str(&format_prec(r, prec + 1));
        },
        Expr::Neg(o) => {
            res.push_str("-");
            res.push_str(&format_prec(o, prec));
        },
        Expr::Pow(l, r) => {
            res.push_str(&format_prec(l, prec + 1));
            res.push_str("^");
            res.push_str(&format_prec(r, prec));
        },
        Expr::Function(name, args) => {
            res.push_str(name);
            res.push('(');
            for (i, a) in args.iter().enumerate() {
                if i != 0 {
                    res.push_str(", ");
                }
                res.push_str(&format_prec(a, 0));
            }
            res.push(')');
        },
        Expr::Variable(name) => {
            res.push_str(name);
        },
        Expr::Literal(s) => {
            res.push_str(s);
        },
    };
    if parent > prec {
        res.push(')');
    }
    return res;
}

