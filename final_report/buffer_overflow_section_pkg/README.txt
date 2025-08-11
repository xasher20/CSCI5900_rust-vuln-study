# Buffer Overflow Section — Reproducible Examples

This package matches the Buffer Overflow section in the report.
It includes three C++ examples and four Rust examples, plus commands to build and run them.

## Files
C++:
- buffer_overflow_1.cpp  -> Out-of-bounds integer array write (unsafe memory write)
- buffer_overflow_2.cpp  -> Character buffer overflow via loop
- buffer_overflow_3.cpp  -> Out-of-bounds read that compiles (undefined behavior)

Rust:
- buffer_overflow_1.rs        -> Runtime panic on out-of-bounds index
- buffer_overflow_2.rs        -> Runtime panic during loop write
- buffer_overflow_3.rs        -> Safe access using .get() (returns Option)
- buffer_overflow_3_compile.rs-> Compile-time error (constant index out of bounds)

## Requirements
- g++ with C++17 or newer
- Rust toolchain (rustc / cargo)
- A shell (macOS Terminal, Linux shell, or Windows WSL)

## Build & Run — C++
g++ -std=c++17 buffer_overflow_1.cpp -o buffer_overflow_1 && ./buffer_overflow_1
g++ -std=c++17 buffer_overflow_2.cpp -o buffer_overflow_2 && ./buffer_overflow_2
g++ -std=c++17 buffer_overflow_3.cpp -o buffer_overflow_3 && ./buffer_overflow_3

## Build & Run — Rust
# Example 1 (runtime panic)
rustc buffer_overflow_1.rs -o buffer_overflow_1 && ./buffer_overflow_1

# Example 2 (runtime panic)
rustc buffer_overflow_2.rs -o buffer_overflow_2 && ./buffer_overflow_2

# Example 3 (safe access)
rustc buffer_overflow_3.rs -o buffer_overflow_3 && ./buffer_overflow_3

# Example 3 (compile-time error demonstration)
rustc buffer_overflow_3_compile.rs -o buffer_overflow_3_compile
# Expected: compilation fails with an index-out-of-bounds error

## Notes
- Outputs may vary for C++ examples due to undefined behavior.
- The Rust examples are written in safe Rust (no `unsafe`), so
  memory-unsafe behavior is prevented by panic or compile-time errors.
