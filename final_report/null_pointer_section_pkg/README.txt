# Null Pointer Dereference Section — Reproducible Examples

This package matches the Null Pointer Dereference section in the report.
It includes C++ examples that crash or invoke undefined behavior and Rust examples
showing safe handling with `Option`, compile-time prevention for uninitialized references,
and a demonstration that UB is only possible via `unsafe` raw pointers.

## Files
C++:
- npd_example1_null_deref.cpp         -> nullptr dereference (segfault)
- npd_example2_uninitialized_ptr.cpp  -> uninitialized pointer dereference (UB)

Rust (safe unless noted):
- npd_example1_option_none.rs         -> safe `Option` handling (no crash)
- npd_example2_unwrap_panic.rs        -> runtime panic when unwrapping None
- npd_example3_no_null_refs.rs        -> compile error if used (no null references allowed)
- npd_example4_unsafe_raw_ptr.rs      -> `unsafe` raw pointer null deref (do not run)

## Build & Run — C++
g++ -std=c++17 npd_example1_null_deref.cpp -o npd1 && ./npd1
g++ -std=c++17 npd_example2_uninitialized_ptr.cpp -o npd2 && ./npd2

## Build & Run — Rust
# Example 1: safe None handling
rustc npd_example1_option_none.rs -o npd_r1 && ./npd_r1

# Example 2: panic when unwrapping None
rustc npd_example2_unwrap_panic.rs -o npd_r2 && ./npd_r2

# Example 3: uncomment the println! to see the compile-time error
rustc npd_example3_no_null_refs.rs -o npd_r3 && ./npd_r3

# Example 4: UNSAFE raw-pointer demo (do not run)
# rustc npd_example4_unsafe_raw_ptr.rs -o npd_r4 && ./npd_r4

## Notes
- C++ allows null and uninitialized pointer dereferences (UB/crashes).
- Safe Rust encodes absence with `Option` and forbids uninitialized references.
- Only `unsafe` raw pointers can express null deref in Rust; this is outside safe Rust.
