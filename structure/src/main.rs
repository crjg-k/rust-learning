struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
// 类单元结构体
struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    println!("{}", user1.email);
    // println!("{}", user1.username);
    println!("{}", user1.active);
    println!("{}", user1.sign_in_count);
    println!("{}", user2.username);

    let black = Color(0, 0, 0);
    let origin = Point(1, 2, 3);
    // 与元组不同的是，解构元组结构体时必须写明结构体的类型。
    let Point(x, y, z) = origin;
    println!("x: {}, y: {}, z: {}", x, y, z);

    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
