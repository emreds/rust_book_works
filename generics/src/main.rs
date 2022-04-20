// We define generics in the function signature like `<x1, x2>`
// This means we can use any type inside the function.
// Caution: `<x, x>` means both should be the same type.
struct Point<x1, y1> {
    x: x1,
    y: y1
}

impl <x1, y1> Point<x1, y1> {
    fn mixup<x2, y2>(self, other: Point<x2, y2>) -> Point<x1, y2> {

        let mixed_point = Point {
            x: self.x,
            y: other.y
        };
        mixed_point
    }
}

fn main() {
    // Here we mix data types.
    let p1 = Point {x: 4, y: 3.2};
    let p2 = Point {x: "hello27", y: "hi42"};
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}