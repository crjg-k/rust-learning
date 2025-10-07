fn main() {
    let data = "initial contents";
    let s = data.to_string();
    println!("s: {s}");
    // 该方法也可直接用于字符串字面值：
    let mut s = "initial contents".to_string();
    println!("s: {s}");

    let v = vec![
        String::from("السلام عليكم"),
        String::from("Dobrý den"),
        String::from("Hello"),
        String::from("שלום"),
        String::from("नमस्ते"),
        String::from("こんにちは"),
        String::from("안녕하세요"),
        String::from("你好"),
        String::from("Olá"),
        String::from("Здравствуйте"),
        String::from("Hola"),
    ];
    for (i, ele) in v.iter().enumerate() {
        println!("{i}th: {}", ele);
    }

    s.push('l');
    let s2 = String::from("world!");
    // 它看起来好像生成了很多拷贝，不过实际上并没有：这个实现比拷贝要更高效。
    let s3 = s + &s2; // 注意 s 被移动了，不能继续使用
    println!("s3: {s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // 宏 format! 生成的代码使用引用因此不会获取任何参数的所有权。
    let s = format!("{s1}-{s2}-{s3}");
    println!("s: {s}");

    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("s: {s}");
    // let s=&hello[0..1];  // 会导致运行时panic

    for c in "Зд".chars() {
        println!("{c}");
    }
    for b in "Зд".bytes() {
        println!("{b}");
    }
}
