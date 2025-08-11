// Demonstrates that use-after-free is only possible in Rust with `unsafe` raw pointers.
// Do NOT run this; for demonstration only.
fn main() {
    let b = Box::new(5);
    let raw = Box::into_raw(b);      // take ownership as raw pointer
    unsafe {
        // memory is freed here
        Box::from_raw(raw);          // drop once
        // use-after-free: dereferencing raw after it was freed
        // println!("{}", *raw);     // UNDEFINED BEHAVIOR if uncommented
    }
}
