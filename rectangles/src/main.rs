#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    // 是`self: &Self`的缩写
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    // 定义不以 self 为第一参数的关联函数（因此不是方法），因为它们并不作用于一个结构体的实例（类似CPP中的静态函数）
    // 这样可以更轻松的创建一个正方形 Rectangle 而不必指定两次同样的值：
    fn square(size: u32) -> Self {
        // 关键字 Self 在函数的返回类型和函数体中，都是对 impl 关键字后所示类型的别名，这里是 Rectangle。
        Self {
            width: size,
            height: size,
        }
    }
}
impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let scale = 2;
    let mut rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    rect1.width = 90;
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(3);
    println!(
        "The area of the sq is {} square pixels.",
        sq.area()
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
