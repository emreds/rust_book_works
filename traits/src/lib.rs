pub mod aggregator {
    use std::fmt::Display;
    // Defining a trait. 
    // We can define a with using `trait` keyword.
    pub trait Summary {
        fn summarize(&self) -> String;
        // Here we define a default function for the trait.
        // If we define a new function with a same name inside the struct, it overrides this function.
        // Default functions differs traits from the interfaces.
        fn country(&self) -> String {
            let country = String::from("Turkey!");
            country
        }
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String 
    }
    // Implementing the trait for the `NewsArticle`.
    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    struct Pair<T> {
        x: T,
        y: T,
    }
    
    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }
    // This is a demonstration of implementing conditional method with a trait.
    // This method only works for the `Pair` which implements `Display + PartialOrd` traits.
    impl <T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x > self.y {
                println!("The largest member is {}", self.x);
            } else if self.x == self.y {
                println!("The members are equal: {} == {}", self.x, self.y);
            } else {
                println!("The largest member is {}", self.y);
            }
        }
        
    }
}