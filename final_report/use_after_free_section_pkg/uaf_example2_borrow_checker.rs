fn main() {
    let r;
    {
        let x = 99;           // Line 5: `x` lives only in this inner scope
        r = &x;               // Line 6: borrow of `x`
    }                          // Line 7: `x` dropped here
    // println!("{}", r);      // Line 8: COMPILE ERROR — `x` does not live long enough
}
