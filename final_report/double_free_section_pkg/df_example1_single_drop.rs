fn main() {
    let p = Box::new(7);
    drop(p);                 // Line 3: moves and drops; memory freed exactly once
    // drop(p);              // Line 4: COMPILE ERROR — use of moved value `p` (prevents double free)
}
