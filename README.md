
MathEval
========

This is a very small rust library for evaluating mathematical expressions. This crate also contains
a small REPL-program as an example of how to use the library (look at `src/main.rs`).

## Usage

### Dependency

Add this library like any other cargo crate to your project. Do this by adding the following to the
`[dependencies]` section of your `Cargo.toml` file.
```ini
matheval = { git = "https://github.com/rolandbernard/matheval" }
```

### Getting started

#### Simple example

The following is a simple implementation of a function that takes a string containing a mathematical
expression as input and tries to compute the result, returning it as a `f64`. If that fails, `None`
is returned.

```rust
use matheval::*;

fn eval(source: &str) -> Option<f64> {
    if let Ok(expr) = Expr::parse(source) {
        if let Ok(res) = expr.eval::<Number>() {
            return Some(res.to_f64());
        }
    }
    return None;
}
```

#### Notes

This library requires you to first parse the expression and then evaluate it. Expressions are
represented using the `matheval::Expr` data type. You can use
`matheval::Expr::parse(&str) -> Result<matheval::Expr, matheval::ParseError>` to parse a given
string into an expression. If the parsing fails, an error of type `matheval::ParseError` will be
returned with a message and a character index describing the error. Error messages are only very
simple and often not very informative.

After having parsed the expression, you can evaluate it using the
`matheval::Expr::eval<T>(&self) -> Result<T, matheval::EvalError>` method.
Evaluation can be done using any type that implements the `matheval::Value` trait. If evaluation fails, an error of type
`matheval::EvalError` is returned, otherwise the result of the evaluation with the given generic type will be returned.

This library also implements already a data type implementing the `matheval::Value` trait. This type
can be found in `matheval::Number` and can represent either a 64 bit floating point value or an
arbitrary precision rational. When used for evaluation this type will try to return rational result
if possible, otherwise fall back to using floating point numbers for functions like `sin`, `log`, etc.
If desired, the result can be converted to an `f64` using the `matheval::Number::to_f64(&self) -> f64`
method.

## Development

The source for the library can be found in the `src/` directory. `src/lib.rs` is the entry point for
the library and `src/main.rs` for a small example program, that evaluates expressions from standard
input.

The `tests/` directory contains various tests for the library.

