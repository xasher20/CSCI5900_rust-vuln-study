fn main() {
    let p = Box::new(42);
    let q = p;              // Line 3: move; `p` is now invalid
    // drop(p);             // Line 4: COMPILE ERROR — use of moved value `p`
    drop(q);                // Line 5: OK: dropped exactly once
}
