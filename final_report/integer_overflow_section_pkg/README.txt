# Integer Overflow/Underflow Section — Reproducible Examples

This package matches the Integer Overflow section in the report.
It includes C++ examples demonstrating signed UB and unsigned wraparound,
and Rust examples showing debug panics, checked/saturating/wrapping APIs,
compile-time detection in const contexts, and safe widen-then-multiply.

## Files
C++:
- int_overflow_example1_signed_ub.cpp     -> signed overflow (undefined behavior)
- int_overflow_example2_unsigned_wrap.cpp -> unsigned wraparound (well-defined)
- int_overflow_example3_multiplies.cpp    -> promote before multiply vs. UB

Rust:
- int_overflow_example1_debug_panic.rs    -> debug panic on overflow; release wraps by default
- int_overflow_example2_checked.rs        -> checked_add returns Option
- int_overflow_example3_saturating.rs     -> saturating_add clamps to bounds
- int_overflow_example4_wrapping.rs       -> wrapping_add wraps explicitly
- int_overflow_example5_const_compile_error.rs -> compile-time error in const context
- int_overflow_example6_widen_then_mul.rs -> safe widening before multiplication

## Build & Run — C++
g++ -std=c++17 int_overflow_example1_signed_ub.cpp -o io1 && ./io1
g++ -std=c++17 int_overflow_example2_unsigned_wrap.cpp -o io2 && ./io2
g++ -std=c++17 int_overflow_example3_multiplies.cpp -o io3 && ./io3

## Build & Run — Rust
# Debug vs Release behavior:
#   Debug: overflow => panic
#   Release: overflow => wrap (unless using checked/explicit APIs)

# Debug (default):
rustc int_overflow_example1_debug_panic.rs -o iro1 && ./iro1

# Checked / Saturating / Wrapping / Widening
rustc int_overflow_example2_checked.rs -o iro2 && ./iro2
rustc int_overflow_example3_saturating.rs -o iro3 && ./iro3
rustc int_overflow_example4_wrapping.rs -o iro4 && ./iro4
rustc int_overflow_example6_widen_then_mul.rs -o iro6 && ./iro6

# Compile-time error demo (const context)
rustc int_overflow_example5_const_compile_error.rs -o iro5
# Expected: compilation fails due to const overflow

## Notes
- C++: Signed overflow is **undefined behavior**; unsigned overflow wraps.
- Rust: Safe indexing/arith can panic in debug and wrap in release; use
  `checked_*`, `saturating_*`, `wrapping_*`, or widen types to get explicit behavior.
- Const-eval in Rust catches overflow at compile time where applicable.
