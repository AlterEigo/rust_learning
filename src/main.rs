#![allow(warnings)]

const HELP_MESSAGE:&str = "
USAGE:

    No options nor arguments yet.
";

fn main() {
    println!("{}", HELP_MESSAGE);
}

fn add(lhs: f64, rhs: f64) -> f64
{
    lhs + rhs
}
