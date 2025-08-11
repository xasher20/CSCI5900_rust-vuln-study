fn main() {
    let r;
    {
        let x = 5;
        r = &x;  // compile-time error: x doesn't live long enough
    }
    // println!("{}", r);
}