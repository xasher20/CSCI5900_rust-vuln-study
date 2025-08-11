// Demonstration only — shows how a data race can be forced with `unsafe` raw pointers.
// DO NOT RUN
static mut COUNTER: i32 = 0;

fn main() {
    let mut handles = vec![];
    for _ in 0..8 {
        handles.push(std::thread::spawn(|| {
            for _ in 0..100_000 {
                unsafe {
                    // DANGEROUS: unsynchronized read-modify-write of a shared static
                    COUNTER += 1; // data race and UB
                }
            }
        }));
    }
    for h in handles { h.join().unwrap(); }
    unsafe { println!("COUNTER={}", COUNTER); }
}
