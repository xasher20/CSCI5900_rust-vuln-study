fn main() {
    let buffer = [1, 2, 3, 4, 5];
    let value = buffer[10]; // compiler error: constant index out of bounds
    println!("Value: {}", value);
}
