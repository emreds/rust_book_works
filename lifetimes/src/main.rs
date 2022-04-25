use std::fmt::Display;

// With this definition `ImportantExcerpt` instance can't outlive
// the `part` field. If there is no `part` there is no
// `ImportantExcerpt`.

struct ImportantExcerpt<'a> {
    part: &'a str,
} 

fn main() {

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    {
        let second_sentence = &first_sentence;
    }
    // This wont work.
    // let ix = ImportantExcerpt {
    //     part: second_sentence
    // };



    let x = String::from("This is a cool string");
    let y = String::from("So much better");
    let ann = String::from("Netflix is losing money today.");

    let result = longest_with_an_announcement(&x, &y, &ann);

}

// There are 3 rules for lifetime decisions of a compiler.
// - Each parameter has its own lifetime.
// fn foo<'a, 'b> (x: &'a i32, y: &'b i32);
// - If there is one input parameter, all output parameters uses
// the lifetime of that one parameter.
// - If there is a `&self` or `&mut self` in the signature
// (this means it's a method) lifetime of the self 
// assigned to all parameters.


// This is a demonstration of the lifetime with a generic type T.
// Output of this function will take the lifetime of x or y.
// It will have the shortest lifetime between x and y.
fn longest_with_an_announcement<'a, T> (
    x: &'a str,
    y: &'a str,
    ann: T
) -> &'a str
where
    T: Display 
    {
        println!("Announcement! {}", ann);

        if x.len() >= y.len() {
            x
        } else {
            y
        }

    }
