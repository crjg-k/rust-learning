fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point1<T> {
    x: T,
    y: T,
}
// 注意必须在 impl 后面声明 T，这样就可以在 Point<T> 上实现的方法中使用 T 了
impl<T> Point1<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
// 可以只为某一具体类型实现方法，这样 Point1<i32> 类就不会有这个方法了
impl Point1<f32> {
    fn _distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Point2<T, U> {
    x: T,
    y: U,
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {result}");

    let _integer = Point1 { x: 5, y: 10 };
    let _float = Point1 { x: 1.0, y: 4.0 };
    // let wont_work = Point1 { x: 5, y: 4.0 };

    let _both_integer = Point2 { x: 5, y: 10 };
    let _both_float = Point2 { x: 1.0, y: 4.0 };
    let _integer_and_float = Point2 { x: 5, y: 4.0 };

    let p = Point1 { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}
// 结构体定义中的泛型类型参数并不总是与结构体方法签名中使用的泛型是同一类型
impl<X1, Y1> Point<X1, Y1> {
    // 而泛型参数 X2 和 Y2 声明于 fn mixup 之后，因为它们只是相对于方法本身的
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
