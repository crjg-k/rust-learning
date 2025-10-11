fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    // 在 Rust 中，迭代器是惰性的（lazy），这意味着在调用消费迭代器的方法之前不会执行任何操作

    for val in v1_iter {
        println!("Got: {val}");
    }
    // for 循环会获取 v1_iter 的所有权并在后台使 v1_iter 可变。
    // for val in v1_iter {
    //     println!("Got: {val}");
    // }

    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    // 注意我们需要将 v1_iter 声明为可变的
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();
    // 调用 sum 之后不再允许使用 v1_iter 因为调用 sum 时它会获取迭代器的所有权。
    assert_eq!(total, 6);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}
