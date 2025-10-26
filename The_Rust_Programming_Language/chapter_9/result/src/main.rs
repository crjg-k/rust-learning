use std::fs::File;
use std::io::{Error, ErrorKind, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let greeting_file_result = File::open("../hello.txt");
    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("../hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };

    let _greeting_file = File::open("../hello.txt").unwrap();
    let _greeting_file =
        File::open("../hello.txt").expect("hello.txt should be included in this project");

    let username_result = read_username_from_file1();
    let username = match username_result {
        Ok(username) => username,
        Err(error) => panic!("read username error1: {error:?}"),
    };
    println!("Username1: {username}");
    let username = read_username_from_file2().unwrap();
    println!("Username2: {username}");
    let username = read_username_from_file3().expect("read username error3");
    println!("Username3: {username}");

    let _greeting_file = File::open("../hello.txt")?;
    Ok(())
}

// 传播错误
fn read_username_from_file1() -> Result<String, Error> {
    let username_file_result = File::open("../username.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// 使用 ? 运算符简化上面的函数
// ? 运算符只能被用于返回值与 ? 作用的值相兼容的函数
fn read_username_from_file2() -> Result<String, Error> {
    let mut username_file = File::open("../username.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// 直接使用链式方法调用来进一步简化
fn read_username_from_file3() -> Result<String, Error> {
    let mut username = String::new();

    File::open("../username.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

// ? 运算符同样可用于 Option 枚举类，但是不可以混合搭配
fn _last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
