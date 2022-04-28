#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

struct Guess {
    value: i32
}

impl Guess {
    pub fn new (value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess{value: value}
    }
}


pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

// Line below indicates that cargo will only compile code below when there is a
// test flag.
// `cargo test -- --show-output` shows the output of the test functions.
// Tests run in parallel by default. To run them in one thread, 
// use `cargo test -- --test-threads=1`.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    // `cargo test larger_can_hold_smaller` only runs the test below.
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 14,
            height: 16
        };

        let smaller = Rectangle { 
            width: 6,
            height: 8
        };

        assert!(larger.can_hold(&smaller))
    }

    #[test]
    fn greeting_contains() {
        let result = greeting("Carol");
        // In this way we can add messages to the test results.
        assert!(
            result.contains("Carol"),
        "Greeting did not contain the name Carol. The received value was  `{}`",
        result );
    }
    // Using `[should_panic(expected=xxxx)]` gives us a flexibility to understand 
    // if test panics in the correct situation.
    #[test]
    #[should_panic(expected="Guess value must be less than or equal to 100")]
    fn greater_than() {
        Guess::new(200);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two gives an error"))
        }
    }
    
    #[test]
    #[ignore]
    // Using `[ignore]` we can ignore some tests in the regular testing case.
    // `cargo test -- --ignored` runs the 'ignored' tests.
    fn expensive_test() {
        println!("Some expensive shit going on here.")
    }

}

