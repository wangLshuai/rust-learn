mod front_of_house;

pub use crate::front_of_house::hosting;
// use self::front_of_house::hosting;
// use crate::front_of_house::hosting::add_to_waitlist;
pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // 改变注意更换想要的面包类型
    meal.toast = String::from("Wheat");

    println!("I'd like {} toast please", meal.toast);

    // 不允许查看或修改早餐附带的季节水果
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    hosting::add_to_waitlist();
    // add_to_waitlist();
}

fn serve_order() {}

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}
