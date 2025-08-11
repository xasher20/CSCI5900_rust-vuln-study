fn main() {
    let a: u32 = u32::MAX;
    let v = a.wrapping_add(1); // Line 3: wraps to 0 explicitly
    println!("wrap={}", v);
}
