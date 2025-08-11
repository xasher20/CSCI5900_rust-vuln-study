fn main() {
    let a: i32 = i32::MAX;
    let b: i32 = 1;
    let v = a.saturating_add(b); // Line 4: saturates at i32::MAX
    println!("saturating sum={}", v);
}
