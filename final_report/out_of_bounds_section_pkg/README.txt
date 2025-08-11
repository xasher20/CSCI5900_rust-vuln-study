# Out-of-Bounds Read/Write Section — Reproducible Examples

This package matches the Out-of-Bounds section in the report.
It includes C++ examples that invoke undefined behavior via out-of-range reads/writes,
and Rust examples that panic safely or use safe APIs to avoid panics.

## Files
C++:
- oob_example1_read.cpp       -> out-of-bounds READ
- oob_example2_write.cpp      -> out-of-bounds WRITE
- oob_example3_loop_read.cpp  -> loop reads past end (i == 3)

Rust:
- oob_example1_read.rs            -> runtime panic on out-of-bounds READ
- oob_example1_read_safe.rs       -> safe indexing via .get()
- oob_example2_write.rs           -> runtime panic on out-of-bounds WRITE
- oob_example3_loop_read.rs       -> runtime panic when loop hits i == 3
- oob_example3_loop_read_safe.rs  -> safe iterator-based sum
- oob_example_const_compile.rs    -> constant index out-of-bounds (compiler error demonstration)

## Build & Run — C++
g++ -std=c++17 oob_example1_read.cpp -o oob1 && ./oob1
g++ -std=c++17 oob_example2_write.cpp -o oob2 && ./oob2
g++ -std=c++17 oob_example3_loop_read.cpp -o oob3 && ./oob3

## Build & Run — Rust
rustc oob_example1_read.rs -o oob_r1 && ./oob_r1
rustc oob_example1_read_safe.rs -o oob_r1s && ./oob_r1s
rustc oob_example2_write.rs -o oob_r2 && ./oob_r2
rustc oob_example3_loop_read.rs -o oob_r3 && ./oob_r3
rustc oob_example3_loop_read_safe.rs -o oob_r3s && ./oob_r3s

# Compile-time error demo (constant index):
rustc oob_example_const_compile.rs -o oob_const
# Expected: compilation fails with index-out-of-bounds error

## Notes
- C++ allows out-of-bounds access on raw arrays; behavior is undefined.
- Safe Rust inserts bounds checks for indexing and provides .get() and iterators to avoid panics.
