fn main() {
    let p = Box::new(7);
    drop(p);                  // Line 3: explicitly drop
    // drop(p);               // Line 4: COMPILE ERROR — value used after move (prevents double free)
}
