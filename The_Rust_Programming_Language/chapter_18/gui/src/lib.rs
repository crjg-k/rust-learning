pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // 1. 由于rust本身不是为了OOP设计的，没有多态，用runtime的稍微的性能损失换取其他OOP语言中的**多态**的实现
    // 2. 这里的 Vec 中可以放置任何实现了 Draw trait 的类型
    // 3. 如果是采用泛型T实现，那么这里的 Vec 中只可以放置一种固定的类型，且在编译时就被确定好
    // 4. 由于 dyn 动态类型无法在编译时已知大小，故需要使用容器来存储（指向）它或者使用引用来引用它F
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("in Button");
    }
}

impl Draw for i32 {
    fn draw(&self) {
        println!("in i32:{}", self);
    }
}
