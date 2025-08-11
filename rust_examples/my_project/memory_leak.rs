use std::mem;

fn main() {
    let leak = Box::new(5);
    mem::forget(leak);  // explicit leak
}