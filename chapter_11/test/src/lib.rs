pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests1 {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // 这样编写测试来返回 Result<T, E> 就可以在函数体中使用问号运算符，如此可以方便的编写任何会返回 Err 的操作的测试。
    #[test]
    fn it_works() -> Result<(), String> {
        let result = add(2, 2);

        if result == 5 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn another() {
        println!("Hello, test!");
        panic!("Make this test fail");
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests2 {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }
}

pub fn add_two(a: usize) -> usize {
    a + 3
}

#[cfg(test)]
mod tests3 {
    use super::*;

    #[test]
    fn it_adds_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }
}
pub fn greeting(_name: &str) -> String {
    String::from("Hello!")
}

#[cfg(test)]
mod tests4 {
    use super::*;

    #[test]
    fn greeting_contains_name1() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }

    #[test]
    fn greeting_contains_name2() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{result}`"
        );
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {value}.");
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {value}.");
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests5 {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100_1() {
        Guess::new(200);
    }

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100_2() {
        Guess::new(200);
    }
}
