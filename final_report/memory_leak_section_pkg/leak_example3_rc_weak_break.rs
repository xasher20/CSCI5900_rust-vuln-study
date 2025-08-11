use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    next: RefCell<Option<Weak<Node>>>, // Line 8: Weak breaks cycles
}

fn main() {
    let a = Rc::new(Node { value: 1, next: RefCell::new(None) });
    let b = Rc::new(Node { value: 2, next: RefCell::new(Some(Rc::downgrade(&a))) });
    *a.next.borrow_mut() = Some(Rc::downgrade(&b)); // Line 14: a ->(Weak) b, b ->(Weak) a
    println!("strong_count a={}, b={}", Rc::strong_count(&a), Rc::strong_count(&b));
    // When main ends, strong counts drop to 0 -> memory freed
}
