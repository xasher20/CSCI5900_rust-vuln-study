fn main() {
    let b = Box::new(42);     // Line 2: allocate on heap
    let moved = b;            // Line 3: move ownership of Box to `moved`
    // println!("{}", b);     // Line 4: COMPILE ERROR — borrow checker: use of moved value `b`
    println!("{}", moved);    // Line 5: OK: value used by its new owner
}
