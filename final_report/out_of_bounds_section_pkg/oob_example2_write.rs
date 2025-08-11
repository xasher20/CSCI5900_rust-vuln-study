fn main() {
    let mut arr = [0; 3];         // Line 2: valid indices 0..2
    arr[5] = 99;                  // Line 3: runtime panic — index out of bounds
    println!("{}", arr[5]);
}
