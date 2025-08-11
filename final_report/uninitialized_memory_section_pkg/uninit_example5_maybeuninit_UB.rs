// Demonstration only — do NOT run. UB if you read before fully initializing.
use std::mem::MaybeUninit;
fn main() {
    let buf: [MaybeUninit<u32>; 3] = unsafe { MaybeUninit::uninit().assume_init() };
    // println!("{:?}", unsafe { std::mem::transmute::<_, [u32; 3]>(buf) }); // UB: reading uninitialized memory
}
