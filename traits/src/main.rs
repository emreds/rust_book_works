use traits::aggregator::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("dogs_ebooks"),
        content: String::from("barkbarkbark! I love barking!"),
        reply: false,
        retweet: false
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("Country of the content: {}", tweet.country());
    notify(&tweet);

    let num_list = vec![1, 5, 77, 99, -3];
    let largest_num = get_largest(&num_list);
    println!("The largest number is {}", largest_num);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let largest_char = get_largest(&char_list);
    println!("The largest char is {}", largest_char);

    let str_list = vec![String::from("blablabla"), String::from("zzzzzzzzzzzzz"), String::from("meowmeow")];
    let largest_str = get_largest(&str_list);
    println!("The largest str is {}", largest_str);

}

// This is the demonstration of using the traits as a type for a function parameter.
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// This is a demonstration of using multiple traits as a type of one parameter of a function. 
// This needs lists which implements `PartialOrd` and `Copy` traits.
// It needs `Copy` trait because in the beginning it copies the first element of the list into the `largest`.
fn get_largest_copy<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list{
        if item > largest {
            largest = item;
        }
    }
    largest
}

// This is a function to find largest number in a list without using copy.
// It gets the reference of the first element in the list and always compares the references with each other.
fn get_largest<T: PartialOrd>(list: &[T]) -> &T {

    let mut largest = &list[0];

    for item in list{
        if &item > &largest {
            largest = &item;
        }
    }

    &largest
}
