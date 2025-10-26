fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}.");

    let mut s = String::from("hello");
    change(&mut s);
    println!("The length of '{s}' is {}.", calculate_length(&s));

    let mut s = String::from("hello");
    {
        let _r1 = &mut s;
    } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用
    let _r2 = &mut s;

    // 下面代码是正确的
    let mut s = String::from("hello");
    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    // 在这里不能出现`let r3 = &mut s;`语句
    println!("{r1} and {r2}");
    // 此位置之后 r1 和 r2 不再使用
    let r3 = &mut s; // 没问题
    println!("{r3}");

    let mut a1 = 5;
    let a2 = &mut a1;
    *a2 = 9;
    {
        // let val_a2 = *a2; // 拷贝一份
        // println!("a1 = {}, a2 = {}", a1, val_a2); // ✅ 可以
        // println!("a1 = {:p}, a2 = {:p}", &a1, &val_a2); //
    }
    {
        println!("a2 points {:p}", a2);
        println!("a2 = {}", *a2);
        println!("a1 addr {:p}", &a1);
        println!("a1 = {}", a1);
    }
}

fn calculate_length(s: &String) -> usize {
    // s 是 String 的引用
    s.len()
} // 这里，s 离开了作用域。但因为它并不拥有引用值的所有权，
// 所以什么也不会发生

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// 悬垂引用问题：
// fn dangle() -> &String { // dangle 返回一个字符串的引用
//
//     let s = String::from("hello"); // s 是一个新字符串
//
//     &s // 返回字符串 s 的引用
// } // 这里 s 离开作用域并被丢弃。其内存被释放。
// // 危险！
