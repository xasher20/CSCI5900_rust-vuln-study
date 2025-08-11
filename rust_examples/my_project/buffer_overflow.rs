fn main() {
    let buffer = [0; 5];
    // buffer[5] = 1;  // compile-time or runtime error
    println!("Buffer: {:?}", buffer);
}