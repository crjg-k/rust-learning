use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let s = Rc::new(RefCell::new("I have multi masters".to_string()));

    let s1 = s.clone();
    let s2 = s.clone();
    // mutable borrow 同样不能同时存在多个
    // let mut s2 = s.borrow_mut();
    s2.borrow_mut().push_str(", oh yeah!");

    println!("{:?}\n{:?}\n{:?}", s, s1, s2);
}
