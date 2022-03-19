#[derive(Debug)]
struct Rectangle {
    width: u32, 
    height: u32
}

// Using `self` keyword, it is always taking the Rectangle as an argument.
impl Rectangle{
    // We use `&` in front of `self` again to not give the ownership to the function.
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        self.width * self.height > rectangle.width * rectangle.height
    }
    // This is called associated function. We can directly call it from the Rectangle struct.
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size, 
            height: size
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!(
        "The area of the rectangle is {}", 
        rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square1 = Rectangle::square(14);

    println!("This is the square {:#?}", square1);
    // This is another Associated Function usage example.
    let a = String::from("newnewnew");
}
