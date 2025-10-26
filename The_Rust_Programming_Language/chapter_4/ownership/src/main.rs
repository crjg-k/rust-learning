fn main() {
    let mut s = String::from("hello");
    s.push_str(", world1"); // push_str() 在字符串后追加字面值
    println!("{s}"); // 将打印 `hello, world!`

    // let s1 = String::from("8964");
    // let s2 = s1;
    // println!("s1 = {s1}, s2 = {s2}");    s1在此不能在被使用，直到重新赋值

    let s1 = String::from("hello");
    let mut s2 = s1.clone();
    s2.push_str(", world2");
    println!("s1 = {s1}, s2 = {s2}");

    let mut s = String::from("2952"); // s 进入作用域
    println!("{}", s);
    takes_ownership(s); // s 的值移动到函数里 ...
    // ... 所以到这里不再有效
    // println!("{}", s); s到此处不再有效，不能直接读取，除非重新赋值
    s = String::from("hello");
    println!("{}", s);

    let x = 5; // x 进入作用域
    makes_copy(x); // x 应该移动函数里，
    // 但 i32 是 Copy 的，
    println!("{}", x); // 所以在后面可继续使用 x

    let x = 89u64;
    let y = x;
    println!("x = {x}, y = {y}");

    let _s1 = gives_ownership(); // gives_ownership 将它的返回值传递给 s1
    let s2 = String::from("hello"); // s2 进入作用域
    let _s3 = takes_and_gives_back(s2); // s2 被传入 takes_and_gives_back,
    // 它的返回值又传递给 s3

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{s2}' is {len}.");
}

fn takes_ownership(some_string: String) {
    // some_string 进入作用域
    println!("{some_string}");
} // 这里，some_string 移出作用域并调用 `drop` 方法。
// 占用的内存被释放

fn makes_copy(some_integer: i32) {
    // some_integer 进入作用域
    println!("some_integer: {some_integer}");
} // 这里，some_integer 移出作用域。没有特殊之处

fn gives_ownership() -> String {
    // gives_ownership 将会把返回值传入调用它的函数
    let some_string = String::from("yours"); // some_string 进入作用域

    some_string // 返回 some_string 并将其移至调用函数
}

// 该函数将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String {
    // a_string 进入作用域
    a_string // 返回 a_string 并移出给调用的函数
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 返回字符串的长度

    (s, length)
}
