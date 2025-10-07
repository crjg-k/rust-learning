enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddr {
    fn output(&self) {
        // println!("{}{}{}{}",s);
        match self {
            IpAddr::V4(ipv4_1, ipv4_2, ipv4_3, ipv4_4) => {
                println!("IP address[{ipv4_1}.{ipv4_2}.{ipv4_3}.{ipv4_4}] is IPv4");
            }
            IpAddr::V6(ipv6) => {
                println!("IP address[{ipv6}] is IPv6");
            }
        };
    }
}

#[derive(Debug)] // 这样可以立刻看到州的名称
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    home.output();
    loopback.output();

    value_in_cents(Coin::Quarter(UsState::Alaska));
}
