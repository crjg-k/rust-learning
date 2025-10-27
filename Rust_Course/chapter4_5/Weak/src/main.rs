use crate::List::{Cons, NULL};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    NULL,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            NULL => None,
        }
    }
}

fn illustrate_loop_ref() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(NULL))));

    println!("a的初始化rc计数 = {}", Rc::strong_count(&a));
    println!("a指向的节点 = {:?}", a.tail());

    // 创建`b`到`a`的引用
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("在b创建后，a的rc计数 = {}", Rc::strong_count(&a));
    println!("b的初始化rc计数 = {}", Rc::strong_count(&b));
    println!("b指向的节点 = {:?}", b.tail());

    // 利用RefCell的可变性，创建了`a`到`b`的引用
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("在更改a后，b的rc计数 = {}", Rc::strong_count(&b));
    println!("在更改a后，a的rc计数 = {}", Rc::strong_count(&a));

    // 下面一行println!将导致循环引用
    // 我们可怜的8MB大小的main线程栈空间将被它冲垮，最终造成栈溢出
    // 这里会递归调用 tail() 函数的原因是因为#[derive(Debug)]，单独只调用是不会发生递归的
    // println!("a next item = {:?}", a.tail());
}

fn main() {
    illustrate_loop_ref();

    // 创建Rc，持有一个值5
    let five = Rc::new(5);
    println!("strong_count = {}", Rc::strong_count(&five));
    println!("weak_count = {}", Rc::weak_count(&five));
    // 通过Rc，创建一个Weak指针
    let weak_five = Rc::downgrade(&five);
    println!("strong_count = {}", Rc::strong_count(&five));
    println!("weak_count = {}", Rc::weak_count(&five));
    // Weak引用的资源依然存在，取到值5
    let strong_five: Option<Rc<_>> = weak_five.upgrade();
    assert_eq!(*strong_five.unwrap(), 5);
    // 手动释放资源`five`
    drop(five);
    // Weak引用的资源已不存在，因此返回None
    let strong_five: Option<Rc<_>> = weak_five.upgrade();
    assert_eq!(strong_five, None);
}
