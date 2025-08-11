# Memory Leak Section — Reproducible Examples

This package matches the Memory Leak section in the report.
It includes C++ examples that leak due to missing delete, pointer overwrites, and shared_ptr cycles,
and Rust examples showing default no-leak behavior, Rc cycles that can leak,
and how to prevent cycles or intentionally leak if desired.

## Files
C++:
- leak_example1_no_delete.cpp           -> new without delete (simple leak)
- leak_example2_overwrite_ptr.cpp       -> pointer overwrite leaks first allocation
- leak_example3_shared_ptr_cycle.cpp    -> cycle leak with shared_ptr
- leak_example3_shared_ptr_cycle_fixed.cpp -> fixed with weak_ptr

Rust:
- leak_example1_raii_drop.rs            -> RAII drop (no leak)
- leak_example2_rc_cycle.rs             -> Rc cycle (leak)
- leak_example3_rc_weak_break.rs        -> Rc + Weak breaks cycle (no leak)
- leak_example4_mem_forget.rs           -> intentional leak with mem::forget

## Build & Run — C++
g++ -std=c++17 leak_example1_no_delete.cpp -o leak1 && ./leak1
g++ -std=c++17 leak_example2_overwrite_ptr.cpp -o leak2 && ./leak2
g++ -std=c++17 leak_example3_shared_ptr_cycle.cpp -o leak3 && ./leak3
g++ -std=c++17 leak_example3_shared_ptr_cycle_fixed.cpp -o leak3_fixed && ./leak3_fixed

## Build & Run — Rust
rustc leak_example1_raii_drop.rs -o leak_r1 && ./leak_r1
rustc leak_example2_rc_cycle.rs -o leak_r2 && ./leak_r2
rustc leak_example3_rc_weak_break.rs -o leak_r3 && ./leak_r3
rustc leak_example4_mem_forget.rs -o leak_r4 && ./leak_r4

## Notes
- C++: Leaks arise from missing deletes, pointer overwrites, and reference cycles using shared_ptr.
- Rust: Ordinary ownership drops values automatically; leaks mainly arise from Rc cycles or explicit leaks
  via mem::forget/Box::leak. Use Weak to break cycles.
