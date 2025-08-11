fn main() {
    let r: &i32;                      // Line 2: references must be initialized before use
    // println!("{}", r);             // Line 3: COMPILE ERROR — use of possibly-uninitialized `r`
}
