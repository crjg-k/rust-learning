struct HasDrop1;
struct HasDrop2;
impl Drop for HasDrop1 {
    fn drop(&mut self) {
        println!("Dropping HasDrop1!");
    }
}
impl Drop for HasDrop2 {
    fn drop(&mut self) {
        println!("Dropping HasDrop2!");
    }
}

struct HasTwoDrops {
    one: HasDrop1,
    two: HasDrop2,
}
// Rust 自动为几乎所有类型都实现了 Drop 特征，因此就算你不手动为结构体实现 Drop，它依然会调用默认实现的 drop 函数
impl Drop for HasTwoDrops {
    fn drop(&mut self) {
        println!("Dropping HasTwoDrops!");
    }
}

struct Foo;
impl Drop for Foo {
    fn drop(&mut self) {
        println!("Dropping Foo!");
    }
}

fn main() {
    // 结构体内部，按照顺序的方式，结构体 _x 中的字段按照定义中的顺序依次 drop
    let _x = HasTwoDrops {
        one: HasDrop1,
        two: HasDrop2,
    };
    let _foo = Foo;
    println!("Running!");
    drop(_foo);
    // 以下代码会报错：借用了所有权被转移的值
    // println!("Running!:{:?}", foo);
}
