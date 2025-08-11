fn main() {
    let ptr: Option<&i32> = None;     // Line 2: explicit absence using Option
    match ptr {                       // Line 3: must handle None explicitly
        Some(v) => println!("{}", v),
        None => println!("Pointer is null (None)"),
    }
}
