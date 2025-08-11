fn main() {
    let arr = [1, 2, 3];
    let sum: i32 = arr.iter().sum();  // SAFE — iterators never go out of bounds
    println!("{}", sum);
}
