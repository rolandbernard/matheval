
use std::io::{stdin, stdout, stderr};
use std::io::prelude::*;

use matheval::Expr;
use matheval::Number;

fn repl_step() -> Result<(), String> {
    let stdin = stdin();
    let stdout = stdout();
    stdout.lock().write(">>> ".as_bytes()).map_err(|err| err.to_string())?;
    stdout.lock().flush().map_err(|err| err.to_string())?;
    let mut input = String::new();
    stdin.lock().read_line(&mut input).map_err(|err| err.to_string())?;
    let expr = Expr::parse(&input).map_err(|err| err.to_string())?;
    let res = expr.eval::<Number>().map_err(|err| err.to_string())?;
    stdout.lock().write(" = ".as_bytes()).map_err(|err| err.to_string())?;
    stdout.lock().write(res.to_string().as_bytes()).map_err(|err| err.to_string())?;
    stdout.lock().write("\n".as_bytes()).map_err(|err| err.to_string())?;
    if res.is_rational() && !res.is_integer() {
        stdout.lock().write(" = ".as_bytes()).map_err(|err| err.to_string())?;
        stdout.lock().write(res.to_f64().to_string().as_bytes()).map_err(|err| err.to_string())?;
        stdout.lock().write("\n".as_bytes()).map_err(|err| err.to_string())?;
    }
    return Ok(());
}

fn main() {
    let stderr = stderr();
    loop {
        if let Err(s) = repl_step() {
            let _ = stderr.lock().write_fmt(format_args!("Error: {}", s));
        }
    }
}

