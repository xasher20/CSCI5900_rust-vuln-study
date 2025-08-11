fn main() {
    let a: i32 = i32::MAX;
    let b: i32 = 1;
    match a.checked_add(b) {     // Line 4: SAFE — returns Option<i32>
        Some(v) => println!("sum={}", v),
        None => println!("overflow detected"),
    }
}
