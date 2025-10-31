use std::fmt;
use std::io::Error;

type Thunk = Box<dyn Fn() + Send + 'static>;

fn _takes_long_type(_: Thunk) {
    // --snip--
}

fn _returns_long_type() -> Thunk {
    Box::new(|| ())
}

type Result<T> = std::result::Result<T, Error>;
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}

fn main() {
    // Kilometers 是 i32 的 同义词，实际上是同一种类型，不同于 newtype 模式
    type Kilometers = i32;
    let x: i32 = 8;
    let y: Kilometers = 9;
    println!("x + y = {}", x + y);

    let f: Thunk = Box::new(|| println!("hi"));
    f();

    generic3("aaa");
    foo("bbb");
}

// 永不返回的 never type
fn _bar() -> ! {
    print!("forever ");

    // 循环永远也不结束，所以此表达式的值是 !。但是如果引入 break 这就不为真了
    loop {
        print!("and ever ");
        // --snip--
        panic!();
    }
}

fn _generic1<T>(_: T) {
    // --snip--
}
// rust会自动为类型实现 Sized trait
fn _generic2<T: Sized>(_: T) {
    // --snip--
}
// ?Sized 这个 trait bound 表示 “T 可以是 Sized，也可以不是 Sized”，注意将参数 t 的类型从 T 变为了 &T
// 具有该含义的 ?Trait 语法仅适用于 Sized，而不适用于其他任何 trait
fn generic3<T: ?Sized>(t: &T)
where
    T: fmt::Display,
{
    println!("t: {t}");
}
fn foo(s: &str) {
    println!("s: {s}");
}
