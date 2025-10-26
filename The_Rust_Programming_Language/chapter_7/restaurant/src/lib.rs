mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn _seat_at_table() {}
    }

    mod serving {
        fn _take_order() {}

        fn _serve_order() {}

        fn _take_payment() {}
    }
}

pub fn eat_at_restaurant1() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}

mod back_of_house1 {
    pub struct Breakfast {
        pub toast: String,
        _seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                _seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant2() {
    // 在夏天订购一个黑麦土司作为早餐
    let mut meal = back_of_house1::Breakfast::summer("Rye");
    // 改变主意更换想要面包的类型
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // 如果取消下一行的注释代码不能编译；
    // 不允许查看或修改早餐附带的季节水果
    // meal.seasonal_fruit = String::from("blueberries");
}

mod back_of_house2 {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant3() {
    let _order1 = back_of_house2::Appetizer::Soup;
    let _order2 = back_of_house2::Appetizer::Salad;
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant4() {
    hosting::add_to_waitlist();
}

mod customer {
    pub fn _eat_at_restaurant() {
        // hosting::add_to_waitlist();  错误，需使用super：
        super::hosting::add_to_waitlist();
    }
}
