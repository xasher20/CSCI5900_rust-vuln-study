fn main() {
    let ptr = Box::new(42);
    drop(ptr);
    // println!("{}", ptr);  // compile-time error
}