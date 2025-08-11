// Returning a reference to a local is rejected at compile time.
fn make_ref<'a>() -> &'a i32 {
    let x = 123;                  // Line 2: local
    // &x                         // If returned, COMPILE ERROR — `x` does not live long enough
    unimplemented!()
}
fn main() { let _ = 0; }
