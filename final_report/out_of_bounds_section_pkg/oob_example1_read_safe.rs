fn main() {
    let arr = [1, 2, 3];
    match arr.get(5) {            // Line 3: SAFE — returns Option<&i32>
        Some(v) => println!("{}", v),
        None => println!("Index out of bounds"),
    }
}
