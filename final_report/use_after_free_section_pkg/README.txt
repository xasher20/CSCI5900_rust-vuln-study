# Use-After-Free Section — Reproducible Examples

This package matches the Use-After-Free section in the report.
It includes three C++ examples that trigger UAF/dangling scenarios and Rust examples
that show compile-time prevention in safe Rust (plus one `unsafe` raw-pointer demo).

## Files
C++:
- uaf_example1_delete_then_use.cpp       -> delete then dereference (UAF)
- uaf_example2_return_local_address.cpp  -> return address of local (dangling / UAF on use)
- uaf_example3_double_free.cpp           -> double free

Rust (safe unless noted):
- uaf_example1_move_then_use.rs          -> use-after-move is prevented (compile error if uncommented)
- uaf_example2_borrow_checker.rs         -> lifetime violation prevented (compile error if uncommented)
- uaf_example3_single_drop.rs            -> double free prevented by ownership
- uaf_example4_unsafe_raw_ptr.rs         -> demonstrates UAF possible only with `unsafe` raw pointers

## Build & Run — C++
g++ -std=c++17 uaf_example1_delete_then_use.cpp -o uaf1 && ./uaf1
g++ -std=c++17 uaf_example2_return_local_address.cpp -o uaf2 && ./uaf2
g++ -std=c++17 uaf_example3_double_free.cpp -o uaf3 && ./uaf3

## Build & Run — Rust
# Example 1: uncomment the erroring line to see compile-time "use of moved value"
rustc uaf_example1_move_then_use.rs -o uaf_r1 && ./uaf_r1

# Example 2: uncomment the erroring println! to see lifetime compile error
rustc uaf_example2_borrow_checker.rs -o uaf_r2 && ./uaf_r2

# Example 3: safe single-drop, uncomment second drop() to see compile error
rustc uaf_example3_single_drop.rs -o uaf_r3 && ./uaf_r3

# Example 4: UNSAFE raw-pointer demo (DO NOT RUN)
# rustc uaf_example4_unsafe_raw_ptr.rs -o uaf_r4 && ./uaf_r4

## Notes
- C++ examples exhibit undefined behavior; outputs may vary or crash.
- Safe Rust prevents UAF at compile time via ownership, moves, and lifetimes.
- Only `unsafe` raw pointers can bypass Rust's guarantees; such code is outside safe Rust.
