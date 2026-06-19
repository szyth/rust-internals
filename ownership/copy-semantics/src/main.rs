#[derive(Copy)] // error: need clone to satisfy generic code requirements
struct Counter {
    value: i32,
}
#[derive(Clone)] // anti pattern: Without Copy, Copy types are forced to behave like Heap-allocated types. A move will invalidate
// previous variable even though the value will still exist in stack
struct Counter2 {
    value: i32,
}

#[derive(Clone, Copy)] // error: label is not Copy type
struct Counter3 {
    value: i32,
    label: String,
}

fn increment(mut n: i32) -> i32 {
    n += 1;
    n
}

fn main() {
    let x = 10;
    let y = increment(x); // silent bitwise copy of x; memcpy
    println!("{x} {y}");
}
