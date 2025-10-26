fn main() {
    // 不是 mut 的必须要指定类型，因为没有向这个 vector 中插入任何值，Rust 并不知道我们想要储存什么类型的元素
    let _v: Vec<i32> = Vec::new();

    let _v = vec![1, 2, 3];

    // Rust 能根据插入的数据做出推断，所以不需要 Vec<i32> 注解
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {third}");
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let v = vec![1, 2, 3, 4, 5];

    // let does_not_exist = &v[99];
    let does_not_exist = v.get(99);
    match does_not_exist {
        Some(hundred) => println!("The 100 element is {hundred}"),
        None => println!("There is no 100 element."),
    }

    // 可变引用的遍历
    let mut v = vec![100, 89, 64];
    for i in &mut v {
        *i *= 10;
    }
    v[0] = 1900;
    // 不可变引用的遍历
    for i in &v {
        println!("{i}");
    }

    // 使用枚举来储存多种类型
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    for ele in row {
        match ele {
            SpreadsheetCell::Int(i) => {
                println!("int: {}", i);
            }
            SpreadsheetCell::Float(f) => {
                println!("float: {}", f);
            }
            SpreadsheetCell::Text(t) => {
                println!("text: {}", t);
            }
        }
    }
}
