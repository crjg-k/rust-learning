use std::thread;
use std::time::Duration;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };
    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );
    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    let _expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    let example_closure = |x| x;
    let _s = example_closure(String::from("hello"));
    // 调用闭包是 add_one_v3 和 add_one_v4 能够编译的必要条件，因为类型将从其用法中推断出来。
    // 当第一次调用推断了类型之后，就不能再以其他类型进行调用
    // let n = example_closure(5);

    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");
    let only_borrows = || println!("From closure: {list:?}");
    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");

    let mut list = vec![8, 9, 6];
    println!("Before defining closure: {list:?}");
    // 当 borrows_mutably 被定义时，它捕获了对 list 的可变引用。闭包在被调用后就不再被使用，这时可变借用结束
    let mut borrows_mutably = || list.push(4);
    borrows_mutably();
    println!("After calling closure: {list:?}");

    // move 关键字可以将所有权移动到闭包当中
    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
    // 到这里，变量 list 已经被move了，不能再被使用了

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];
    // 闭包被多次调用，所以不能实现为 FnOnce（会将捕获的值从闭包体中移出）的闭包
    // list.sort_by_key(|r| r.width);
    // println!("{list:#?}");

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{list:#?}, sorted in {num_sort_operations} operations");

    /*
       注意：如果我们要做的事情不需要从环境中捕获值，则可以在需要某种实现了 Fn trait 的东西时使用函数而不是闭包。
       举个例子，可以在 Option<Vec<T>> 的值上调用 unwrap_or_else(Vec::new)，以便在值为 None 时获取一个新
       的空的 vector。编译器会自动为函数定义实现适用的 Fn trait。
    */
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
