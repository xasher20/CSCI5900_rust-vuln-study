use std::rc::Rc;
fn main() {
    let a = Rc::new(String::from("hello"));
    let b = Rc::clone(&a);
    let c = Rc::clone(&a);
    // All clones share ownership; memory freed once when the last owner goes out of scope.
    println!("counts: strong={}, weak={}", Rc::strong_count(&a), Rc::weak_count(&a));
}
