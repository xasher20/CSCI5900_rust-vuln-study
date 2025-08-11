fn main() {
    let mut buffer = [0u8; 5];
    for i in 0..10 {
        buffer[i] = b'A';  // panic when i >= 5
    }
    println!("Buffer: {:?}", buffer);
}
