// buffer_overflow_1_runtime.rs
use std::env;

fn main() {
    let buffer = [0; 5];
    let i: usize = env::args()
        .nth(1)
        .unwrap_or_else(|| "10".into()) // default to 10 if no arg passed
        .parse()
        .expect("index must be a number");
    let value = buffer[i]; // will panic at runtime if i >= 5
    println!("Buffer: {}", value);
}
