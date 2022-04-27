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

fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

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
        assert!(result.contains("Carol"));
    }

}

