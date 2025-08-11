// Demonstration only — using raw pointers can create dangling references.
// DO NOT RUN
fn main() {
    let mut v = vec![1, 2, 3];
    let p: *const i32 = &v[0];       // raw pointer into vector buffer
    v.reserve(1000);                  // may reallocate
    unsafe {
        // println!("{}", *p);       // DANGEROUS — `p` may dangle after reallocation (UB)
    }
}
