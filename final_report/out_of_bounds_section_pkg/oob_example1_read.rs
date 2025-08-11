fn main() {
    let arr = [1, 2, 3];          // Line 2: valid indices 0..2
    println!("{}", arr[5]);       // Line 3: runtime panic — index out of bounds
}
