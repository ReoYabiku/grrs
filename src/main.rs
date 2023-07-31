mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fuit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fuit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheet");
    println!("I'd like {} toast please!", meal.toast);
}

fn main() {
    eat_at_restaurant();
}