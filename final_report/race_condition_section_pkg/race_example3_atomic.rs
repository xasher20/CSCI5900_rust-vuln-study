use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::sync::Arc;

fn main() {
    let counter = Arc::new(AtomicUsize::new(0));     // Line 6: atomic shared counter
    let mut handles = vec![];
    for _ in 0..8 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..100_000 {
                c.fetch_add(1, Ordering::Relaxed);   // Line 12: lock-free increment
            }
        }));
    }
    for h in handles { h.join().unwrap(); }
    println!("counter={}", counter.load(Ordering::Relaxed)); // Deterministic: 800000
}
