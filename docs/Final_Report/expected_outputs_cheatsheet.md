
# Expected Outputs Cheat Sheet (C++ vs Rust)
Use this as a quick reference when running the examples from each ZIP. Notes marked “(varies)” indicate undefined behavior (UB) or environment-dependent output. Panic line numbers may differ slightly by compiler version.

---

## 1) Buffer Overflow
- C++ Example 1 (`buffer_overflow_1.cpp`): Often prints a value (e.g., `Buffer: 42`) or garbage; may crash. (varies, UB)
- Rust Example 1 (`buffer_overflow_1.rs`): **Panics** — `index out of bounds: the len is 5 but the index is 10`.
- C++ Example 2 (`buffer_overflow_2.cpp`): Prints overflown characters or garbage (e.g., `Buffer content: AAAAA...`), may crash. (varies, UB)
- Rust Example 2 (`buffer_overflow_2.rs`): **Panics** at `i == 5` — out-of-bounds index.
- Rust Example 3 (`buffer_overflow_3.rs`): Prints `Index out of bounds` (safe `.get()`).
- Rust Example 3 Compile (`buffer_overflow_3_compile.rs`): **Compile-time error** — constant index out of bounds.

## 2) Use-After-Free (UAF)
- C++ Ex1 (`uaf_example1_delete_then_use.cpp`): May print old value, garbage, or crash. (varies, UB)
- Rust Ex1 (`uaf_example1_move_then_use.rs`): If the commented line is enabled → **Compile error** “use of moved value”.
- C++ Ex2 (`uaf_example2_return_local_address.cpp`): May print garbage or crash. (varies, UB)
- Rust Ex2 (`uaf_example2_borrow_checker.rs`): If println is enabled → **Compile error** “does not live long enough”.
- C++ Ex3 (`uaf_example3_double_free.cpp`): Often crash or allocator error. (varies, UB)
- Rust Ex3 (`uaf_example3_single_drop.rs`): Runs and exits quietly; uncomment second `drop` → **Compile error**.
- Rust Ex4 (`uaf_example4_unsafe_raw_ptr.rs`): (Do not run) Uncommenting raw deref can cause UB/crash.

## 3) Null Pointer Dereference
- C++ Ex1 (`npd_example1_null_deref.cpp`): Segmentation fault likely. (varies, UB)
- Rust Ex1 (`npd_example1_option_none.rs`): Prints `Pointer is null (None)`.
- C++ Ex2 (`npd_example2_uninitialized_ptr.cpp`): Crash or garbage. (varies, UB)
- Rust Ex3 (`npd_example3_no_null_refs.rs`): If println enabled → **Compile error** (uninitialized reference).
- Rust Ex2 (`npd_example2_unwrap_panic.rs`): **Panics** — `called Option::unwrap() on a None value`.
- Rust Ex4 (`npd_example4_unsafe_raw_ptr.rs`): (Do not run) Uncommenting raw deref can cause UB/crash.

## 4) Out-of-Bounds Read/Write
- C++ Ex1 (`oob_example1_read.cpp`): Prints garbage or crashes. (varies, UB)
- Rust Ex1 (`oob_example1_read.rs`): **Panics** — index out of bounds.
- Rust Ex1 Safe (`oob_example1_read_safe.rs`): Prints `Index out of bounds`.
- C++ Ex2 (`oob_example2_write.cpp`): May print garbage or crash; memory corruption possible. (varies, UB)
- Rust Ex2 (`oob_example2_write.rs`): **Panics** on write.
- C++ Ex3 (`oob_example3_loop_read.cpp`): Prints incorrect sum or crashes. (varies, UB)
- Rust Ex3 (`oob_example3_loop_read.rs`): **Panics** when `i == 3`.
- Rust Ex3 Safe (`oob_example3_loop_read_safe.rs`): Prints `6`.

## 5) Double Free
- C++ Ex1 (`df_example1_delete_twice.cpp`): Often crashes or allocator error. (varies, UB)
- Rust Ex1 (`df_example1_single_drop.rs`): Runs quietly; uncomment second `drop` → **Compile error**.
- C++ Ex2 (`df_example2_alias_delete.cpp`): Often crashes. (varies, UB)
- Rust Ex2 (`df_example2_move_alias.rs`): Runs quietly; uncomment `drop(p)` → **Compile error**.
- C++ Ex3 (`df_example3_malloc_free_twice.cpp`): Often crashes or reports error. (varies, UB)
- Rust Ex3 (`df_example3_rc_safe.rs`): Prints counts like `strong=3, weak=0`.
- Rust Ex4 (`df_example4_unsafe_raw_double_free.rs`): (Do not run) Uncommenting second from_raw() → UB/double free.

## 6) Memory Leaks
- C++ Ex1 (`leak_example1_no_delete.cpp`): Prints `42`; leak persists (no visible error).
- Rust Ex1 (`leak_example1_raii_drop.rs`): Prints `42`; memory freed on scope exit.
- C++ Ex2 (`leak_example2_overwrite_ptr.cpp`): Prints `2`; first allocation leaked silently.
- C++ Ex3 (`leak_example3_shared_ptr_cycle.cpp`): No destructor prints (indicates leak).
- C++ Ex3 Fixed (`leak_example3_shared_ptr_cycle_fixed.cpp`): Prints `~Node` lines on exit.
- Rust Ex2 (`leak_example2_rc_cycle.rs`): Prints strong counts > 0 (leak).
- Rust Ex3 (`leak_example3_rc_weak_break.rs`): Prints strong counts (normal), memory freed on exit.
- Rust Ex4 (`leak_example4_mem_forget.rs`): No output; leaks intentionally.

## 7) Race Condition (Data Race)
- C++ Ex1 (`race_example1_no_mutex.cpp`): `counter=` less than 800000 (nondeterministic). (varies, UB)
- Rust Ex1 (`race_example1_compile_error.rs`): If increment enabled → **Compile error** (cannot share &mut across threads).
- Rust Mutex (`race_example2_arc_mutex.rs`): `counter=800000` (deterministic).
- Rust Atomic (`race_example3_atomic.rs`): `counter=800000` (deterministic).
- C++ Ex2 (`race_example2_read_write_flag.cpp`): Worker may spin too long or terminate late. (varies)
- Rust Unsafe (`race_example4_unsafe_raw.rs`): (Do not run) UB if executed.

## 8) Uninitialized Memory
- C++ Ex1 (`uninit_example1_stack_var.cpp`): Prints garbage or crashes. (varies, UB)
- Rust Ex1 (`uninit_example1_compile_error.rs`): If println enabled → **Compile error**.
- C++ Ex2 (`uninit_example2_uninit_array.cpp`): Prints garbage sum or crashes. (varies, UB)
- Rust Ex3 (`uninit_example3_array_default.rs`): Prints `0`.
- C++ Ex3 (`uninit_example3_new_no_init.cpp`): Prints garbage value. (varies, UB)
- Rust Ex2 (`uninit_example2_init_before_use.rs`): Prints `10`.
- Rust MaybeUninit Safe (`uninit_example4_maybeuninit.rs`): Prints `[1, 2, 3]`.
- Rust MaybeUninit UB (`uninit_example5_maybeuninit_UB.rs`): (Do not run) UB if uncommented.

## 9) Dangling Pointers
- C++ Ex1 (`dangling_example1_return_local.cpp`): Prints garbage or crashes. (varies, UB)
- Rust Ex1 (`dangling_example1_borrow_checker.rs`): If returning `&x` → **Compile error** (lifetime).
- C++ Ex2 (`dangling_example2_vector_realloc.cpp`): Prints garbage or crashes after push. (varies, UB)
- Rust Ex2 (`dangling_example2_vec_borrow_then_push.rs`): If push enabled → **Compile error** (borrow rules).
- Rust Ex3 (`dangling_example3_index_or_clone.rs`): Prints `1` safely.
- C++ Ex3 (`dangling_example3_free_then_later_use.cpp`): Prints garbage or crashes. (varies, UB)
- Rust Unsafe (`dangling_example4_unsafe_raw.rs`): (Do not run) UB if deref uncommented after reallocation.

## 10) Integer Overflow / Underflow
- C++ Signed (`int_overflow_example1_signed_ub.cpp`): Prints unexpected/negative or odd behavior. (varies, UB)
- Rust Debug (`int_overflow_example1_debug_panic.rs`): **Panics** — attempt to add with overflow.
- Rust Release (same file): Wraps silently; program prints `done`.
- Rust Checked (`int_overflow_example2_checked.rs`): Prints `overflow detected`.
- Rust Saturating (`int_overflow_example3_saturating.rs`): Prints `saturating sum=2147483647`.
- Rust Wrapping (`int_overflow_example4_wrapping.rs`): Prints `wrap=0`.
- C++ Widen First (`int_overflow_example3_multiplies.cpp`): Prints `p=2500000000 q=<garbage or UB>`.
- Rust Widen (`int_overflow_example6_widen_then_mul.rs`): Prints `p=2500000000`.
- Rust Const (`int_overflow_example5_const_compile_error.rs`): **Compile-time error**: overflow in const.

---

## Tips
- **UB outputs vary**: crashes, strange values, or “works” are all possible. That’s normal.
- **Panic locations** may show different file/line numbers; the message content is what matters.
- For Rust **debug vs. release**: `rustc` default is debug; `cargo run --release` uses release.
- If a Rust example says “Do not run,” keep it for the report but don’t execute it.

