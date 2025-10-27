use std::cell::Cell;

// Cell 只适用于 Copy 类型，用于提供值，而 RefCell 用于提供引用
// Cell 不会 panic，而 RefCell 会
fn main() {
    let c = Cell::new("asdf");
    // Cell 类型针对的是实现了 Copy trait 的值类型，故调用 `get()` 会实现复制的操作
    let one = c.get();
    c.set("qwer");
    let two = c.get();
    println!("{},{}", one, two);
}
