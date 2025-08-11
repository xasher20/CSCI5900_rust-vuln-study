use std::mem::MaybeUninit;
fn main() {
    // SAFE usage pattern: write all fields before reading
    let mut buf: [MaybeUninit<u32>; 3] = unsafe { MaybeUninit::uninit().assume_init() };
    for i in 0..3 {
        buf[i] = MaybeUninit::new(i as u32 + 1);   // initialize each element
    }
    let arr: [u32; 3] = unsafe { std::mem::transmute(buf) }; // all initialized -> safe
    println!("{:?}", arr);
}
