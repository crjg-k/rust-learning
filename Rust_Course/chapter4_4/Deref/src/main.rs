use std::ops::{Deref, DerefMut};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn main() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let y = MyBox::new(5);
    assert_eq!(5, *y);

    let s = String::from("hello world");
    // 仅引用类型的实参才会触发自动解引用，且仅对于函数和方法的传参
    display(&s);
    display(&*s);

    // 连续的隐式 Deref 转换，一下四种调用方式均等价
    let s = MyBox::new(String::from("hello world"));
    display(&s);
    display(&*s);
    display(&**s);
    display(&(*s)[..]);

    let s = MyBox::new(String::from("hello, world"));
    // 赋值操作需要手动解引用
    let _s1: &str = &s;
    // 方法调用会自动解引用
    let _s2: String = s.to_string();
    let _s2: String = (*s).to_string();
}

fn display(s: &str) {
    println!("{}", s);
}
