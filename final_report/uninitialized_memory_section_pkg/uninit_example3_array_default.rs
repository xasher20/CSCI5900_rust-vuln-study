fn main() {
    let arr = [0i32; 3];          // Line 2: fully initialized array
    let sum: i32 = arr.iter().sum();
    println!("{}", sum);
}
