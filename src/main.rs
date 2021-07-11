#![allow(warnings)]

const HELP_MESSAGE:&str = "
USAGE:

    No options nor arguments yet.
";

fn main() {
    println!("{}", HELP_MESSAGE);
}

fn add(lhs: f64, rhs: f64) -> f64 { lhs + rhs }
fn sub(lhs: f64, rhs: f64) -> f64 { lhs - rhs }
fn div(lhs: f64, rhs: f64) -> f64 { lhs / rhs }
fn mul(lhs: f64, rhs: f64) -> f64 { lhs * rhs }
fn modulo(lhs: f64, rhs: f64) -> f64 { lhs % rhs }
