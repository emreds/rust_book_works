use std::collections::HashMap;

fn main() {
    // vectors();
    // strings();
    // hashMaps();
}

fn vectors() {
    let v: Vec<i32> = Vec::new();

    // Annotation below infers vector's data type automatically.
    let v = vec![1, 2, 3];

    let mut v = Vec::new();
    // If I don't push any value, compiler gives an error.
    // Because, it cannot infer the data type of the vector `v`.
    v.push(5);
    v.push(6);

    let v = vec![1, 2, 3, 4, 5];

    // This gives an error(panic) when the index of the element is not in v.
    let third: &i32 = &v[2];
    
    // This does not give an error when the index is not in v.
    match v.get(2) {
        Some(third) => println!("This is the value of number {}", third),
        None => println!("It is not matching with anything")
    }

    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];

    // If we push and try to print `first` compiler panics.
    // Since the first is not mutable, after pushing an element vector might
    // need to allocate new memory and copy the current vector into a new space.
    // In this case `first` might be pointing to deallocated space.
    // It doesn't matter if the `first` is `mut` or not.
    
    // v.push(6);
    // println!("This is the value of first {}", first);
    
    for i in &v {
        println!("This is the element {}", i);
    }

    for i in &mut v {
        // I need to work on dereferencing
        *i += 50;
        println!("This is `i` {}", i);

    }

    println!("This is the v after modification {:?}", v);

    enum SpreadSheetcell {
        Int(i32),
        Float(f64),
        Text(String)
    }

    let row = vec![
        SpreadSheetcell::Int(3),
        SpreadSheetcell::Float(6.4),
        SpreadSheetcell::Text(String::from("Trial"))
    ];

    let last = v.pop();

    println!("This is the last element of v {:?}", last);
}

fn strings() {

    let data = "Initial contents";
    // To convert the data into string, it should implement the `Display` trait.
    let s = data.to_string();
    // Below code same with `to_string`.
    let s = String::from("Initial contents");
    
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    // push_str does not take ownership.
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // String add function requires `&` reference of the `s2`.
    let s3 = s1 + &s2;
    println!("s2 is {}", s2);
    // println!("s1 is {}", s1);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // Possible to use `format!`.
    let s = s1 + "-" + &s2 + "-" + "-" + &s3;
    
    println!("This is s3 {}", s3);
    println!("This is s2 {}", s2);
    // It panics while printing `s1` cause the ownership of s1 passes to `add` function.
    // println!("This is s1 {}", s1);
    // Because the value of `s1` is dropped, we need to define it again.
    let s1 = String::from("tic");
    let s = format!("{}-{}-{}", s1, s2, s3);
    
    println!("This is s3 {}", s3);
    println!("This is s2 {}", s2);
    // While using `format!` ownership of the s1 doesn't pass to the `add` function.
    println!("This is s1 {}", s1);

    // String indexing is not possible in rust `s1[0]`
}

fn hashMaps() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 30);

    for (key, value) in &scores {
        println!("Key and value in scores {}, {}", key, value);
    }

    let teams = vec![String::from("Blue"), String::from("Red")];
    let initial_scores = vec![10, 30];

    // Zipping two vectors into HashMap.
    let scores: HashMap<_,_> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();
    
    println!("This is the zipped scores {:?}", scores);

    let field = String::from("Spiderman");
    let power = 99;
    let mut map = HashMap::new();
    // `insert` takes the ownership of `field` and `power`.
    // Those variables are not usable after inserting.
    map.insert(field, power);
    // `insert` overwrites onto the value.
    map.insert(String::from("Spiderman"), 100);
    println!("{:?}", map);

    // Only insert if there is no value 
    let mut points = HashMap::new();
    points.insert("Jake", 100);
    points.insert("Michael", 120);

    points.entry("Michael").or_insert(130);
    points.entry("George").or_insert(50);

    println!("This is points hashmap {:?}", points);

    // Updating the value based on old value.
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("This is the text map {:?}", map);
}
