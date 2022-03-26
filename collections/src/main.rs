fn main() {
    
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
    
    // This does not gave an error when the index is not in v.
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
        //*i += 50;
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
