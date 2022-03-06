
use std::io::{stdin, stdout, stderr};
use std::io::prelude::*;

use matheval::Expr;
use matheval::Quantity;

fn repl_step() -> Result<(), String> {
    let stdin = stdin();
    let stdout = stdout();
    stdout.lock().write(">>> ".as_bytes()).map_err(|err| err.to_string())?;
    stdout.lock().flush().map_err(|err| err.to_string())?;
    let mut input = String::new();
    stdin.lock().read_line(&mut input).map_err(|err| err.to_string())?;
    if let Some(idx) = input.find(" to ") {
        let expr = Expr::parse(&input[..idx]).map_err(|err| err.to_string())?;
        let res = expr.eval::<Quantity>().map_err(|err| err.to_string())?;
        let dest = input[idx + 4..].trim();
        if let Some(result) = res.convert_to(dest) {
            stdout.lock().write_fmt(format_args!(" = {} {}\n", result.to_string(), dest)).map_err(|err| err.to_string())?;
            if result.is_rational() && !result.is_integer() {
                stdout.lock()
                    .write_fmt(format_args!(" = {} {}\n", result.to_f64(), dest))
                    .map_err(|err| err.to_string())?;
            }
        } else {
            return Err(format!("Cannot convert {} to {}", res.unit().to_string(), dest));
        }
    } else {
        let expr = Expr::parse(&input).map_err(|err| err.to_string())?;
        let res = expr.eval::<Quantity>().map_err(|err| err.to_string())?;
        stdout.lock()
            .write_fmt(format_args!(" = {} {}\n", res.coefficient().to_string(), res.unit().to_string()))
            .map_err(|err| err.to_string())?;
        if res.coefficient().is_rational() && !res.coefficient().is_integer() {
            stdout.lock()
                .write_fmt(format_args!(" = {} {}\n", res.coefficient().to_f64(), res.unit().to_string()))
                .map_err(|err| err.to_string())?;
        }
    }
    return Ok(());
}

fn main() {
    let stderr = stderr();
    loop {
        if let Err(s) = repl_step() {
            let _ = stderr.lock().write_fmt(format_args!("Error: {}\n", s));
        }
    }
}

