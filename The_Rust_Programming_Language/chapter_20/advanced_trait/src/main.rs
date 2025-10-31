use std::fmt::Display;

// 1. 由于 trait 当中不能含有字段，所以无法通过指定字段的值来推断 trait 的泛型参数的类型来使编译器完成单态化
// 2. 所以在 trait 上基本只能使用和结构体泛型参数强绑定、强关联的泛型参数，这就大大削弱了 trait 的灵活性和可用范围
trait Graph1<N, E> {
    fn edges1(&self, node: &N) -> Vec<E>;
}
struct MyGraph1<N, E> {
    x: N,
    y: E,
}
// 这里为了实现 edges1 函数中的打印功能，限定了泛型 N 的类型
impl<N, E> Graph1<N, E> for MyGraph1<N, E>
where
    N: Display,
{
    fn edges1(&self, node: &N) -> Vec<E> {
        println!("node: {}", node);
        vec![]
    }
}

trait Graph2 {
    type Node;
    type Edge;
    fn edges2(&self, node: &Self::Node) -> Vec<Self::Edge>;
}
struct MyGraph2<N, E> {
    x: N,
    y: E,
}
// 这里进要求关联类型 Node 是可打印的即可，与泛型参数 N 解耦，提高了 trait 的灵活性和可用范围
impl<N, E> Graph2 for MyGraph2<N, E> {
    type Node = u32;
    type Edge = (u32, u32);

    fn edges2(&self, node: &Self::Node) -> Vec<Self::Edge> {
        println!("node: {}", node);
        vec![]
    }
}

fn main() {
    let a = MyGraph1 { x: 8u32, y: 9u32 };
    a.edges1(&6);

    let c = MyGraph2 { x: 8u32, y: 9u32 };
    c.edges2(&6);
}
