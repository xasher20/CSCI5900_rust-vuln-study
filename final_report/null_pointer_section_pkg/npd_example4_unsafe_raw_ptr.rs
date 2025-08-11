// Demonstration only — do not run.
fn main() {
    let p: *const i32 = std::ptr::null(); // null raw pointer
    unsafe {
        // println!("{}", *p);           // DANGEROUS — null deref via raw pointer (UB)
    }
}
