mod front_of_house;

mod back_of_house {
    // pub on a struct doesn't implicitly make all its fields pub: in fact,
    // they all remain private unless otherwise stated.
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

    // In contrast to structs, making an enum public makes all of its
    // variants public.
    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub fn fix_incorrect_order() {
        cook_order();
        super::front_of_house::serving::serve_order();
    }

    fn cook_order() {
        Breakfast {
            toast: String::from("Rye"),
            // We can see the struct's private field even outside of its impl,
            // being in the same module.
            seasonal_fruit: String::from("gomu-gomu no mi"),
        };
        ()
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    hosting::add_to_waitlist(); // use is applied to the scope, not from its source line onwards
}

// Use of use ;)

// Imports module here and exports it as public as well
pub use crate::front_of_house::hosting;
// Alternatively, these do not re-export the hosting module to others:
// use crate::front_of_house::hosting;
// use self::front_of_house::hosting;

pub fn eat_at_restaurant_2() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

use std::fmt::Result;
use std::io::Result as IoResult; // alias to solve same-name problem

// Multiple use declarations in one
// use std::{cmp::Ordering, io}; // 
// use std::io::{self, Write}; //same as: use std::io; use std::io::Write

use std::collections::*; // glob operator
