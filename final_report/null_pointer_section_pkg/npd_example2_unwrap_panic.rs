fn main() {
    let ptr: Option<i32> = None;      // Line 2: None
    println!("{}", ptr.unwrap());      // Line 3: RUNTIME PANIC — called `Option::unwrap()` on a `None` value
}
