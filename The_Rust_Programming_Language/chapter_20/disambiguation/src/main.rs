trait Pilot {
    fn fly(&self);
}
trait Wizard {
    fn fly(&self);
}
struct Human;
impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}
impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}
impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}
struct Dog;
impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}
impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    println!("A baby dog is called a {}", Dog::baby_name());
    // 因为 Animal::baby_name 没有 self 参数，而且可能有其他类型实现了 Animal trait，
    // Rust 无法确定我们想调用哪一个 Animal::baby_name 的实现：
    // println!("A baby dog is called a {}", Animal::baby_name());

    // 完全限定语法
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}
