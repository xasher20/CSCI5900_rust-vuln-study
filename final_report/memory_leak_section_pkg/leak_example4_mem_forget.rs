fn main() {
    let s = String::from("leak me");
    std::mem::forget(s); // Line 3: intentionally leak by forgetting to drop
    // Note: Box::leak can also intentionally leak:
    // let _leaked: &'static mut String = Box::leak(Box::new(String::from("leaked")));
}
