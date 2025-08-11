# CSCI Independent Study — *Rust vs. C++: Preventing Common Software Vulnerabilities*

**Author:** Xen Hasnat, University of Colorado Boulder
**Advisor:** Professor Ahmed Hamza

## Overview

This repository accompanies the report *Rust vs. C++: Preventing Common Software Vulnerabilities*.
It contains **fully reproducible C++ and Rust examples** for ten common vulnerability classes, each with equivalent implementations to illustrate how Rust’s safety features prevent or mitigate them compared to C++.

The code is organized by vulnerability type, with **matching C++ and Rust examples**, a `README.txt` in each section folder, and a set of **terminal run screenshots** for select cases.

---

## Vulnerabilities Covered

1. Buffer Overflow
2. Use-After-Free
3. Null Pointer Dereference
4. Out-of-Bounds Read/Write
5. Double Free
6. Memory Leak
7. Race Condition (Data Race)
8. Uninitialized Memory
9. Dangling Pointer
10. Integer Overflow

---

## Repository Structure
CSCI5900_rust-vuln-study/final_report
```
/buffer_overflow_section_pkg/      # Buffer Overflow examples (C++ and Rust)
/use_after_free_section_pkg/       # Use-After-Free examples
/null_pointer_section_pkg/         # Null Pointer Dereference examples
/out_of_bounds_section_pkg/        # Out-of-Bounds Read/Write examples
/double_free_section_pkg/          # Double Free examples
/memory_leak_section_pkg/          # Memory Leak examples
/race_condition_section_pkg/       # Race Condition examples
/uninitialized_memory_section_pkg/ # Uninitialized Memory examples
/dangling_pointer_section_pkg/     # Dangling Pointer examples
/integer_overflow_section_pkg/     # Integer Overflow examples
expected_outputs_cheatsheet.md     # Quick reference of expected outputs
README.md                          # This file
```

Each section folder contains:

* **C++ source files** (`.cpp`)
* **Rust source files** (`.rs`)
* `README.txt` with compilation & execution instructions
* **Screenshots** folder showing actual terminal runs for selected examples

---

## Compilation & Execution

### C++ Examples

Compile with `g++` (or `clang++`):

```bash
g++ -std=c++17 filename.cpp -o output
./output
```

### Rust Examples

Compile with `rustc`:

```bash
rustc filename.rs
./filename
```

---

## Expected Output

* Each example source file includes **commented expected output** based on the report.
* Actual runs may vary slightly by **platform** or **compiler version**, but the **key differences in safety behavior between C++ and Rust remain consistent**.
* For convenience, see `expected_outputs_cheatsheet.md` for a consolidated summary.

---

## Platform Notes

All examples were tested on:

* macOS (Apple Clang for C++)
* Rust 1.x stable

Behavior may vary slightly on Linux or Windows, but vulnerabilities and Rust’s safety checks behave consistently across platforms.

---

## CWE Mappings (External Links)

Where applicable, vulnerabilities are mapped to **Common Weakness Enumeration (CWE)** IDs in the report, for example:

* [CWE-787: Out-of-Bounds Write](https://cwe.mitre.org/data/definitions/787.html)
* [CWE-416: Use After Free](https://cwe.mitre.org/data/definitions/416.html)
* [CWE-476: NULL Pointer Dereference](https://cwe.mitre.org/data/definitions/476.html)
* …and others per section.

---

## Reproducibility

This repository, along with the final PDF report, ensures full reproducibility:

1. Clone or download this repository.
2. Follow the compilation instructions in each section’s `README.txt`.
3. Compare terminal outputs to the screenshots and `expected_outputs_cheatsheet.md` located in docs/Final_Report/

---
