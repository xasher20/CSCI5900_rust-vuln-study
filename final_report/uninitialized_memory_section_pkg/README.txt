# Uninitialized Memory Section — Reproducible Examples

This package matches the Uninitialized Memory section in the report.
It includes C++ programs that read uninitialized stack/heap memory (UB),
and Rust programs that show compile-time prevention, safe initialization patterns,
and `MaybeUninit` for low-level initialization control.

## Files
C++:
- uninit_example1_stack_var.cpp     -> uninitialized local variable
- uninit_example2_uninit_array.cpp  -> uninitialized array read in loop
- uninit_example3_new_no_init.cpp   -> uninitialized heap `new int`

Rust:
- uninit_example1_compile_error.rs  -> compile error on uninitialized use
- uninit_example2_init_before_use.rs-> initialize then use (OK)
- uninit_example3_array_default.rs  -> arrays initialized with default
- uninit_example4_maybeuninit.rs    -> safe `MaybeUninit` pattern
- uninit_example5_maybeuninit_UB.rs -> demonstration of UB if misused (commented)

## Build & Run — C++
g++ -std=c++17 uninit_example1_stack_var.cpp -o u1 && ./u1
g++ -std=c++17 uninit_example2_uninit_array.cpp -o u2 && ./u2
g++ -std=c++17 uninit_example3_new_no_init.cpp -o u3 && ./u3

## Build & Run — Rust
# Compile-error demonstration (uncomment to see error)
rustc uninit_example1_compile_error.rs -o ur1 && ./ur1

rustc uninit_example2_init_before_use.rs -o ur2 && ./ur2
rustc uninit_example3_array_default.rs -o ur3 && ./ur3
rustc uninit_example4_maybeuninit.rs -o ur4 && ./ur4
# Do not run the UB demonstration
# rustc uninit_example5_maybeuninit_UB.rs -o ur5 && ./ur5

## Notes
- C++ permits reading uninitialized memory (UB).
- Safe Rust forbids use-before-initialize at compile time.
- For low-level patterns, use `MaybeUninit` carefully: write-all-before-read.
