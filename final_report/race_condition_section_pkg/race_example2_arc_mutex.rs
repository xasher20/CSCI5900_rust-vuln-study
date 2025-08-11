use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));     // Line 5: shared, synchronized state
    let mut handles = vec![];
    for _ in 0..8 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..100_000 {
                let mut guard = c.lock().unwrap(); // Line 11: lock before mutate
                *guard += 1;
            }
        }));
    }
    for h in handles { h.join().unwrap(); }
    println!("counter={}", *counter.lock().unwrap()); // Deterministic: 800000
}
