struct Point1 {
    x: i32,
    y: i32,
}

struct Point2 {
    x: i32,
    y: i32,
    z: i32,
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor1(i32, i32, i32),
    ChangeColor2(Color),
}

fn main() {
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50, outer y = {}", y),
        // 这里会存在变量遮蔽，y是用来匹配x的一个“占位符”，在这条分语句的作用域中并不是值为10的那个变量y
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {x:?}"),
    }
    println!("at the end: x = {x:?}, y = {y}");

    // 多模式匹配
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // 范围匹配，范围只允许用于数字或 char 值
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    let p = Point1 { x: 6, y: 4 };
    let Point1 { x: a, y: b } = p;
    assert_eq!(6, a);
    assert_eq!(4, b);
    let Point1 { x, y } = p;
    assert_eq!(6, x);
    assert_eq!(4, y);
    match p {
        Point1 { x, y: 0 } => println!("On the x axis at {x}"),
        Point1 { x: 0, y } => println!("On the y axis at {y}"),
        Point1 { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }

    let msg = Message::ChangeColor1(0, 160, 255);
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor1(r, g, b) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        _ => (),
    }
    let msg = Message::ChangeColor2(Color::Hsv(0, 160, 255));
    match msg {
        Message::ChangeColor2(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message::ChangeColor2(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}");
        }
        _ => (),
    }

    let ((_feet, _inches), Point1 { x, y }) = ((3, 10), Point1 { x: 3, y: -10 });

    foo(3, 4);

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {setting_value:?}");

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}");
        }
    }

    // 1. 只使用下划线本身，并不会绑定值。下面的代码段可以正确编译，因为 s 没有被移动进 _：
    // 2. 但是如果是使用的 _s 来进行匹配，就会发生s的移动从而阻止我们继续打印s的值
    let s = Some(String::from("Hello!"));
    if let Some(_) = s {
        println!("found a string");
    }
    println!("{s:?}");

    // 用 .. 忽略剩余值
    let origin = Point2 { x: 0, y: 0, z: 0 };
    match origin {
        Point2 { x, .. } => println!("x is {x}"),
    }
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }
    // 使用 .. 必须是无歧义的，下面的代码会报错
    // match numbers {
    //     (.., second, ..) => {
    //         println!("Some numbers: {second}")
    //     }
    // }

    // 匹配守卫，可以用来解决变量遮蔽问题
    let num = Some(4);
    match num {
        Some(x) if (x % 2 == 0) => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {x:?}"),
    }
    println!("at the end: x = {x:?}, y = {y}");
    let x = 4;
    let y = false;
    match x {
        // (4 | 5 | 6) if y => ... 是正确的理解
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    enum Message2 {
        Hello { id: i32 },
    }
    let msg = Message2::Hello { id: 5 };
    match msg {
        // 这里的分支中，id 字段的值会被绑定到临时变量 id_variable 上
        Message2::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {id_variable}"),
        // 这里的分支无法使用到 id 的值了
        Message2::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message2::Hello { id } => println!("Found some other id: {id}"),
    }
}

fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {y}");
}
