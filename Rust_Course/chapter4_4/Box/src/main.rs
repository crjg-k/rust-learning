fn main() {
    let a = Box::new(3);
    println!("a = {}", a); // a = 3
    // 下面一行代码将报错
    // let b = a + 1; // cannot add `i32` to `Box<i32>`

    // 在栈上创建一个长度为1000的数组
    let arr = [0; 1000];
    // 将arr所有权转移arr1，由于 `arr` 分配在栈上，因此这里实际上是直接重新深拷贝了一份数据
    let arr1 = arr;
    // arr 和 arr1 都拥有各自的栈上数组，因此不会报错
    println!("{:?}", arr.len());
    println!("{:?}", arr1.len());

    // 在堆上创建一个长度为1000的数组，然后使用一个智能指针指向它
    let arr = Box::new([0; 1000]);
    // 将堆上数组的所有权转移给 arr1，由于数据在堆上，因此仅仅拷贝了智能指针的结构体，底层数据并没有被拷贝
    // 所有权顺利转移给 arr1，arr 不再拥有所有权
    let arr1 = arr;
    println!("{:?}", arr1.len());
    // 由于 arr 不再拥有底层数组的所有权，因此下面代码将报错
    // println!("{:?}", arr.len());

    let arr = vec![Box::new(1), Box::new(2)];
    let (first, second) = (&arr[0], &arr[1]);
    let sum = **first + **second;
    println!("sum = {}", sum);

    let config = gen_static_config();
    println!("config.name = {}", config.name);
}

fn _gen_static_str() -> &'static str {
    let mut s = String::new();
    s.push_str("hello, world");

    Box::leak(s.into_boxed_str())
}

struct Config {
    name: &'static str,
}

fn gen_static_config() -> &'static Config {
    let a = Box::new(Config { name: "abcd" });

    Box::leak(a)
}
