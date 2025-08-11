fn main() {
    let arr = [1, 2, 3];
    let mut sum = 0;
    for i in 0..=3 {               // Line 4: DANGEROUS — i==3 is out of bounds
        sum += arr[i];             // Line 5: runtime panic when i==3
    }
    println!("{}", sum);
}
