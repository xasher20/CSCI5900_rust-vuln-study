// Demonstration only — do not run. Shows how double free can be reintroduced with `unsafe`.
fn main() {
    let b = Box::new(123);
    let raw = Box::into_raw(b);        // take ownership as raw pointer
    unsafe {
        // Convert back once and drop (frees memory):
        let _once = Box::from_raw(raw);
        // Convert back again and drop a second time — DOUBLE FREE if uncommented:
        // let _twice = Box::from_raw(raw);
    }
}
