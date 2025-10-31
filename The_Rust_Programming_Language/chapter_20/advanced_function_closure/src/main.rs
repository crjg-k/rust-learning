fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

#[derive(Debug)]
enum Status {
    // 定义的每一个枚举成员也变成了一个构造函数
    Value(u32),
    Stop,
}

fn main() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {answer}");

    let list_of_numbers = vec![1, 2, 3];
    let _list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    let list_of_numbers = vec![1, 2, 3];
    let _list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();

    let list_of_statuses: Vec<Status> = (0u32..20u32).map(Status::Value).collect();
    for item in list_of_statuses {
        println!("{item:?}");
    }

    let handlers = vec![returns_closure2(), returns_initialized_closure(64)];
    for handler in handlers {
        let output = handler(89);
        println!("{output}");
    }
}

fn _returns_closure1() -> impl Fn(i32) -> i32 {
    |x| x + 1
}
// 套上一层 Box<dyn ...> 后，returns_closure2()和returns_initialized_closure() 的返回类型就相同了
fn returns_closure2() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn returns_initialized_closure(init: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x + init)
}
