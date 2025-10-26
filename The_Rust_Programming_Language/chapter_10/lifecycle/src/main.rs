use std::fmt::Display;

// 在结构体定义中使用生命周期参数
struct ImportantExcerpt<'a> {
    // 这个注解意味着 ImportantExcerpt 的实例不能比其 part 字段中的引用存在的更久。
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn _level(&self) -> i32 {
        8964
    }
}

impl<'a> ImportantExcerpt<'a> {
    fn _announce_and_return_part1<'b>(&self, announcement: &'b str) -> &'b str {
        println!("Attention please: {announcement}");
        announcement
    }
}

impl<'a, 'b> ImportantExcerpt<'a> {
    fn _announce_and_return_part2(&self, announcement: &'b str) -> &'b str {
        println!("Attention please: {announcement}");
        announcement
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest1(string1.as_str(), string2.as_str());
        println!("The longest string is \"{result}\"");
    }
    // 这里 result 所引用的变量可能是已经无效的 string2，所以无法通过借用检查器的检查
    // println!("The longest string is {result}");

    // 只有在 first_sentence 还有效时，结构体字段才能引用其。以下报错：
    // let first_sentence;
    // {
    //     let novel = String::from("Call me Ishmael. Some years ago...");
    //     first_sentence = novel.split('.').next().unwrap();
    // }
    // let _i = ImportantExcerpt {
    //     part: first_sentence,
    // };

    // 所有的字符串字面值都拥有 'static 生命周期，其生命周期能够存活于整个程序期间
    let _s: &'static str = "I have a static lifetime.";
}

// 函数或方法的参数的生命周期被称为 输入生命周期，而返回值的生命周期被称为 输出生命周期。
// 为了在函数签名中使用生命周期注解，需要在函数名和参数列表间的尖括号中声明泛型生命周期（lifetime）参数，就像泛型类型（type）参数一样。
fn longest1<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// 不返回第二个参数就无需为其指定泛型生命周期
fn _longest2<'a>(x: &'a str, _y: &str) -> &'a str {
    x
}

// 因为生命周期也是泛型，所以生命周期参数 'a 和泛型类型参数 T 都位于函数名后的同一尖括号列表中。
fn _longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}
