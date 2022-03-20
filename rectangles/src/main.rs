// By default it is not possible to print struct Rectangle by debug.
// Thus, we add `#[derive(Debug)]` line before it.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 30
    };

    println!("This is the area of the rectangle {}", area(&rect1));

    // Trying to print struct.

    println!("This is the rectangle we have {:#?}", rect1);

    let scale = 2; 
    let rect2 = Rectangle {
        height: 50, 
        width: dbg!(scale * 30)
    };
    // We use borrowing here because, rect2 is stored in the heap and if we pass it to dbg!
    // It will take the ownership and drop the value.
    dbg!(&rect2);

    println!("This is rect2 {:#?}", rect2);
}


fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
