// Elision Rules: Rule 1 -> Rule 2
// fn a<'a>(x: &'a str) -> &'a str
fn a(x: &str) -> &str {
    todo!()
}

// Elision rules: Rule 1. Rule 2 not applicable, usize aint reference
// fn b<'a, 'b>(x: &'a str, y: &'b str) -> usize {
fn b(x: &str, y: &str) -> usize {
    todo!()
}

// Error: Only Elision Rule 1 is applicable, not enough for compiler to infer output lifetime.
fn c(x: &str, y: &str) -> &str {
    todo!()
}

fn main() {
    println!("Hello, world!");
}
