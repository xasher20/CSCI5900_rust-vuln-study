fn main() {
    let x: i32 = 50_000;
    let y: i32 = 50_000;
    let p: i64 = (x as i64) * (y as i64); // Line 4: safe widen-then-multiply
    // let q: i32 = x * y;               // Line 5: in debug, panic; in release, wrap (avoid)
    println!("p={}", p);
}
