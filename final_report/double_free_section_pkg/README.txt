# Double Free Section — Reproducible Examples

This package matches the Double Free section in the report.
It includes C++ programs that perform double free via direct and aliasing cases,
and Rust programs that show compile-time prevention via ownership and `Rc` for shared ownership.
An `unsafe` example demonstrates how double free can be reintroduced only with raw pointers.

## Files
C++:
- df_example1_delete_twice.cpp       -> delete p; delete p;
- df_example2_alias_delete.cpp       -> alias two raw pointers; both delete
- df_example3_malloc_free_twice.cpp  -> free twice with malloc/free

Rust (safe unless noted):
- df_example1_single_drop.rs         -> single drop; second drop would be compile error
- df_example2_move_alias.rs          -> move semantics prevent alias double drop
- df_example3_rc_safe.rs             -> `Rc` shows safe shared ownership
- df_example4_unsafe_raw_double_free.rs -> `unsafe` raw-pointer double free demo (do not run)

## Build & Run — C++
g++ -std=c++17 df_example1_delete_twice.cpp -o df1 && ./df1
g++ -std=c++17 df_example2_alias_delete.cpp -o df2 && ./df2
g++ -std=c++17 df_example3_malloc_free_twice.cpp -o df3 && ./df3

## Build & Run — Rust
# Example 1: uncomment second drop to see compile-time error
rustc df_example1_single_drop.rs -o df_r1 && ./df_r1

# Example 2: uncomment drop(p) to see compile-time error on moved value
rustc df_example2_move_alias.rs -o df_r2 && ./df_r2

# Example 3: Rc shared ownership (freed once on last drop)
rustc df_example3_rc_safe.rs -o df_r3 && ./df_r3

# Example 4: UNSAFE raw-pointer double free (do not run)
# rustc df_example4_unsafe_raw_double_free.rs -o df_r4 && ./df_r4

## Notes
- C++ double free is undefined behavior and often exploitable.
- Safe Rust prevents double free at compile time via moves and single-drop semantics.
- Shared ownership in Rust uses `Rc`/`Arc` reference counting to avoid double free.
- Only `unsafe` raw-pointer conversions can reintroduce double free in Rust; this is opt-in.
