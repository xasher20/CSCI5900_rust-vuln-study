use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    next: RefCell<Option<Rc<Node>>>,
}

fn main() {
    let a = Rc::new(Node { value: 1, next: RefCell::new(None) });
    let b = Rc::new(Node { value: 2, next: RefCell::new(Some(a.clone())) });
    *a.next.borrow_mut() = Some(b.clone()); // Line 15: create cycle a -> b -> a

    println!("strong_count a={}, b={}", Rc::strong_count(&a), Rc::strong_count(&b));
    // Both Rc counts stay > 0 at end of main -> memory is leaked (no Drop runs)
}
