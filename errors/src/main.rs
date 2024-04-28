use std::fs::{read_to_string, File};
use std::io::ErrorKind;
use std::io;
use std::io::Read;

/* 
enum Result<T, E> {
    Ok(T),
    Err(E),
}
*/

fn main() {
    //a();
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
            },
            other_error => {
                panic!("There was a problem opening the file: {:?}", other_error)
            }
        }
    };

    // Using closures
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Tried to create file but there was a problem: {:?}", error)
            })
        } else {
            panic!("There was a problem opening the file: {:?}", error)
        }
    });

    // Shortcuts for panic on error: unwrap and expect
    let f = File::open("hello.txt").unwrap();
    let f = File::open("hello.txt").expect("Failed to open hello.txt");

    // Propagating errors
    let f = read_username_from_file();
    match f {
        Ok(s) => println!("{}", s),
        Err(e) => println!("Error: {}", e),
    }


}

fn a() {
    b();
}

fn b() {
    c(22);
}

fn c(num: i32) {
    if num == 22 {
        panic!("Don't pass in 22!");
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}