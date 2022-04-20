use std::error::Error;
use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

//
fn main() {
    // To backtrace, use cargo run with `RUST_BACKTRACE=1`

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt"){
                Ok(fc) => fc,
                Err(err) => panic!("Problem while creating a file {:?}", err),
            }
            other_err => panic!("Error in file reading {:?}", other_err)
        }
    };

    // With unwrap or else we can write many things in to file.
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem while creating a file {:?}", error);
            })
        } else {
            panic!("Error in file reading {:?}", error);
        }
    });

    // This will return the error if there is an error.
    let f = File::open("hello.txt").unwrap();
    // `expect` keyword let's us to use speficic panic message.
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e)
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e)
    }
}

// `?` Only be used with functions which return `Result<T, E>` or `Option<T>`.
fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

// Does the same thing with the functions above but with using a library.
fn read_username_from_file_shortest() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// Usage of ? with Option.
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
