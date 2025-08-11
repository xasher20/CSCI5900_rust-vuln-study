fn main() {
    let arr = [0; 3];
    let x = arr[10];            // Expected: compile-time error when proven constant OOB
    println!("{}", x);
}
