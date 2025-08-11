fn main() {
    let a: i32 = i32::MAX;       // Line 2
    let b: i32 = 1;
    let _c = a + b;              // Line 4: debug builds panic; release builds wrap by default
    println!("done");
}
