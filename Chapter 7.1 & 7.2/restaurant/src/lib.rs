//mod front_of_house {
//    mod hosting {
//        fn add_to_waitlist() {}
//
//        fn seat_at_table() {}
//    }
//
//    mod serving {
//        fn take_order() {}
//
//        fn serve_order() {}
//
//        fn take_payment() {}
//    }
//}
//
//mod back_of_house {
//    pub struct Breakfast {
//        pub toast: String,
//        seasonal_fruit: String,
//    }
//
//    impl Breakfast {
//        pub fn summer(toast: &str) -> Breakfast {
//            Breakfast {
//                toast: String::from(toast),
//                seasonal_fruit: String::from("peaches"),
//            }
//        }
//    }
//}
//
//pub fn eat_at_restaurant() {
//    // Order a breakfast in the summer with Rye toast
//    let mut meal = back_of_house::Breakfast::summer("Rye");
//    // Change our mind about what bread we'd like
//    meal.toast = String::from("Wheat");
//    println!("I'd like {} toast please", meal.toast);
//
//    // The next line won't compile if we uncomment it; we're not allowed
//    // to see or modify the seasonal fruit that comes with the meal
//    // meal.seasonal_fruit = String::from("blueberries");
//}
//
/*
--Module Tree--
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
*/
