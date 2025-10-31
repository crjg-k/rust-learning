// 1. 函数参数列表也可以使用模式匹配
// 2. 闭包类似于函数，也可以在闭包参数列表中使用模式。
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location1: ({x}, {y})");
}

fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        for val in [1, 2, 3] {
            tx.send(val).unwrap();
        }
    });
    // while let 是 if let 的循环版本
    while let Ok(value) = rx.recv() {
        println!("{value}");
    }

    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{value} is at index {index}");
    }

    let mut point = (8, 9);
    let foo = |(x, y): &(i32, i32)| println!("Current location2: ({x}, {y})");
    print_coordinates(&point);
    foo(&point);
    point = (6, 4);
    print_coordinates(&point);
    foo(&point);

    // 1. IDE推断的类型：b - &i32, c - i32, e - &i32
    // 2. 这其实是rust模式匹配的一种形式，模式里的 & 是按引用形状解构，把一层引用“剥掉”再绑定内部值
    let a = 1;
    let b = &a;
    let &c = &a;
    let &d = &a;
    let &e = &b;
    // 3. 必须是实现了 Copy 的才可以使用引用模式匹配的结构，这件事本质上不同于直接使用let语句的move移动
    // 4. &a可以被用来多次作为右值出现在let语句的等号右边，因为a本身不会被move
    // let a = String::from("test rust");
    // let b = &a;
    // let c = a;
    // let &c = &a;
}
