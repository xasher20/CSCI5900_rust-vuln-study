fn main() {
    let mut v = vec![1, 2, 3];
    let x = v[0];                  // Line 3: copy the value (i32: Copy)
    v.push(4);                     // Line 4: can mutate now; no outstanding borrow
    println!("{}", x);             // Line 5: safe; uses the copied value
}
