# Dangling Pointers Section — Reproducible Examples

This package matches the Dangling Pointers section in the report.
It includes C++ examples that create and use dangling pointers, and Rust examples
that show compile-time prevention via the borrow checker, plus safe alternatives.
An `unsafe` demo illustrates how dangling can only be reintroduced in Rust with raw pointers.

## Files
C++:
- dangling_example1_return_local.cpp        -> return address of local (dangling on return)
- dangling_example2_vector_realloc.cpp      -> pointer invalidated by vector reallocation
- dangling_example3_free_then_later_use.cpp -> free then later use through saved pointer

Rust:
- dangling_example1_borrow_checker.rs       -> returning reference to local is rejected
- dangling_example2_vec_borrow_then_push.rs -> borrow then push shows compile-time error
- dangling_example3_index_or_clone.rs       -> safe pattern: copy/index then mutate
- dangling_example4_unsafe_raw.rs           -> `unsafe` raw pointer dangling demo (do not run)

## Build & Run — C++
g++ -std=c++17 dangling_example1_return_local.cpp -o d1 && ./d1
g++ -std=c++17 dangling_example2_vector_realloc.cpp -o d2 && ./d2
g++ -std=c++17 dangling_example3_free_then_later_use.cpp -o d3 && ./d3

## Build & Run — Rust
# Examples demonstrate compile-time errors when you uncomment the indicated lines.
rustc dangling_example1_borrow_checker.rs -o dr1 && ./dr1
rustc dangling_example2_vec_borrow_then_push.rs -o dr2 && ./dr2
rustc dangling_example3_index_or_clone.rs -o dr3 && ./dr3
# Unsafe demo — do not run
# rustc dangling_example4_unsafe_raw.rs -o dr4 && ./dr4

## Notes
- C++ allows creation and use of dangling pointers; behavior is undefined.
- Safe Rust prevents dangling references with lifetimes and borrow rules.
- Only explicit `unsafe` raw pointer code can produce dangling in Rust.
