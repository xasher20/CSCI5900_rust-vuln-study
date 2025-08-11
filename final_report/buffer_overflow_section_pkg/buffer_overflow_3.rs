fn main() {
    let buffer = [1, 2, 3, 4, 5];
    match buffer.get(10) {
        Some(val) => println!("Value: {}", val),
        None => println!("Index out of bounds"),
    }
}
