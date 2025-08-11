fn main() {
    let p = Box::new(42);   // Line 2: allocation
    println!("{}", p);
    // Line 4: p is dropped automatically at end of scope -> no leak
}
