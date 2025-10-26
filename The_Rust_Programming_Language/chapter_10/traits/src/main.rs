use std::fmt::{Debug, Display};

// 默认实现允许调用相同 trait 中的其他方法，哪怕这些方法没有默认实现
pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    // 注释掉以使用 trait 的默认实现
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }
    fn summarize_author(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

fn main() {
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    };
    println!("1 new post: {}", post.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };
    println!("New article available! {}", article.summarize());

    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    };
    println!("1 new social post: {}", post.summarize());

    notify1(&post);

    let s = 2952.to_string();
    println!("{}", s);
}

// 该参数支持任何实现了指定 trait 的类型
// 任何用其它如 String 或 i32 的类型调用该函数的代码都不能编译，因为它们没有实现 Summary
pub fn notify1(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
// 上面其实是语法糖，更展开的写法是如下的：
pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
// 这适用于 item1 和 item2 允许是不同类型的情况（只要它们都实现了 Summary）
pub fn notify3(_item1: &impl Summary, _item2: &impl Summary) {}
// 泛型 T 被指定为 item1 和 item2 的参数限制，如此传递给参数 item1 和 item2 值的具体类型必须一致。
pub fn notify4<T: Summary>(_item1: &T, _item2: &T) {}

// 通过 + 号实现&&的逻辑
pub fn notify5(_item: &(impl Summary + Display)) {}
pub fn notify6<T: Summary + Display>(_item: &T) {}

// 使用 where 从句后移 trait bound
fn _some_function<T, U>(_t: &T, _u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    8964
}

// 也可以在返回值中使用 impl Trait 语法，来返回实现了某个 trait 的类型：
fn _returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    }
}

// 通过使用带有 trait bound 的泛型参数的 impl 块，可以有条件地只为那些实现了特定 trait 的类型实现方法。
struct Pair<T> {
    x: T,
    y: T,
}
impl<T> Pair<T> {
    fn _new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
impl<T: Display + PartialOrd> Pair<T> {
    fn _cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
