
use num::Num;

fn main() {
    let v = matheval::expr::value::Value::from_str_radix("123/1", 10).unwrap();
    let u = v.clone() + v.clone();
    println!("Hello, world! {}", u.to_f64());
}

