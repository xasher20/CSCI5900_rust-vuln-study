// This example shows a compile-time error: multiple mutable accesses across threads.
fn main() {
    let mut counter = 0;                        // Line 3: local variable, not Send/Sync by default for &mut
    let mut handles = Vec::new();
    for _ in 0..8 {
        // Attempt to capture &mut counter across threads -> COMPILE ERROR
        handles.push(std::thread::spawn(|| {
            for _ in 0..100_000 {
                // counter += 1; // Uncommenting would fail to compile: cannot capture `counter` by reference across threads
            }
        }));
    }
    for h in handles { h.join().unwrap(); }
    println!("{}", counter);
}
