use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

// 我们手动实现的 HelloMacro trait 及其关联函数
// impl HelloMacro for Pancakes {
//     fn hello_macro() {
//         println!("Hello, Macro! My name is Pancakes!");
//     }
// }

fn main() {
    Pancakes::hello_macro();
    let a = stringify!(test stringify);
    println!("a: \"{a}\"");
}
