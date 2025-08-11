# Race Condition (Data Race) Section — Reproducible Examples

This package matches the Race Condition section in the report.
It includes C++ examples that exhibit data races without synchronization and corrected versions,
and Rust examples that show compile-time prevention, safe synchronization with Arc<Mutex<..>>,
lock-free atomics, and an explicit `unsafe` raw-pointer race demonstration.

## Files
C++:
- race_example1_no_mutex.cpp        -> increment shared counter without mutex (data race)
- race_example2_read_write_flag.cpp -> unsynchronized read/write of a shared flag
- race_example3_with_mutex.cpp      -> fixed using std::mutex

Rust:
- race_example1_compile_error.rs    -> compile-time prevention (cannot share &mut across threads)
- race_example2_arc_mutex.rs        -> safe shared mutable state via Arc<Mutex<_>>
- race_example3_atomic.rs           -> safe lock-free increment using AtomicUsize
- race_example4_unsafe_raw.rs       -> explicit `unsafe` data race using static mut (do not run)

## Build & Run — C++
g++ -std=c++17 -pthread race_example1_no_mutex.cpp -o r1 && ./r1
g++ -std=c++17 -pthread race_example2_read_write_flag.cpp -o r2 && ./r2
g++ -std=c++17 -pthread race_example3_with_mutex.cpp -o r3 && ./r3

## Build & Run — Rust
# Example 1 intentionally contains commented lines that would not compile if enabled.
rustc race_example1_compile_error.rs -o rr1 && ./rr1

rustc race_example2_arc_mutex.rs -o rr2 && ./rr2
rustc race_example3_atomic.rs -o rr3 && ./rr3

# UNSAFE demo (do not run)
# rustc race_example4_unsafe_raw.rs -o rr4 && ./rr4

## Notes
- C++ exhibits true data races that are UB unless synchronization is used.
- Safe Rust prevents data races at compile time; use Arc<Mutex<_>> or atomics for shared mutable state.
- `unsafe` raw pointer patterns can reintroduce data races in Rust and must be carefully audited.
