mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() { 
            add_to_waitlist(); 
        }
            
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
    
}

fn serve_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String
    }
    
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }
    
    pub enum Appetizer {
        Soup, 
        Salad
    }
    
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}

pub fn eat_at_restaurant() {
    // Absolute path 
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Wheat");

    meal.toast = String::from("Corn");

    println!("I'd like {} toast please", meal.toast);
    // Line below will give an error
    // meal.seasonal_fruit = String::from("Blackberry");

    // For enums we don't need to define every field as pub explicitly.
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

use self::front_of_house::hosting;

// This function is here to demonstrate usage of `use` keyword.
pub fn eat_at_restaurant_2(){
    hosting::add_to_waitlist();
}
// If we are bringing structs and enums with use we specify the full path,
// Instead of bringing parent module. 

use std::collections::HashMap;

fn demonstrate(){
    let mut map = HashMap::new();
    map.insert(1,2);
}

// If we are going to use two items with the same name, we need to use them with parents. 

use std::fmt;
use std::io;

// fn function1() -> fmt::Result {}
// fn function2() -> io::Result<()> {}

// Using the Glob operator 
// use std::collections::*;
// Using nested lists
// use std::{cmp::Ordering, io};
// Line below brings the `io` and `Write`
// use std::io::{self, Write};

use rand::Rng;

fn rand() {
    let secret_number = rand::thread_rng().gen_range(1..101);
}
