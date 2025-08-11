fn main() {
    let buffer = [0; 5];                // fixed-size array
    let value = buffer[10];             // causes panic at runtime
    println!("Buffer: {}", value);
}
