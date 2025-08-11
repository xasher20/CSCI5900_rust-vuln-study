fn main() {
    let mut v = vec![1, 2, 3];
    let r = &v[0];                 // Line 3: immutable borrow of element
    // v.push(4);                  // Line 4: COMPILE ERROR — cannot borrow `v` as mutable because it is also borrowed as immutable
    println!("{}", r);             // Line 5: safe; borrow ends here
}
