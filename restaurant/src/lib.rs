mod back_of_house{
    pub struct Breakfast{
        pub toast:String,
        pub seasonal_fruit:String,
    }

    impl Breakfast{
        pub fn summer(toast:&str)->Breakfast{
            Breakfast{
                toast:String::from(toast),
                seasonal_fruit:String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant(){
    let mut meal=back_of_house::Breakfast::summer("Rye");

    let orange=back_of_house::Breakfast{
        toast:String::from("Rye"),
        seasonal_fruit:String::from("oranges"),
    };

    meal.toast=String::from("Wheat");
    println!("{}",meal.toast);
}