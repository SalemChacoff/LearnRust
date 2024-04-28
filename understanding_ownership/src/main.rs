

fn main() {

    // variable scope
    {
        let s = "hello";
        println!("{}", s);
    }


    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);      // s's value moves into the function...
                                        // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);        // x would move into the function,
                        

    // Use of clone to create a deep copy
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("{}, world!", s2); // error: value borrowed here after move

    let t1 = String::from("hello");

    let (t2, len) = calculate_length(t1);

    println!("The length of '{}' is {}.", t2, len);


    let x1 = String::from("hello");

    let len2 = calculate_length_borrowing(&x1);

    println!("The length of '{}' is {}.", x1, len2);

    // Mutable references
    let mut s = String::from("hello");

    change(&mut s);

    println!("{}", s);

    // Mutable references have one big restriction: you can have only one mutable reference to a particular piece of data in a particular scope.
    // This code will not compile because it has two mutable references:
    // let mut s = String::from("hello");

    // let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{}, {}", r1, r2);

    // Dangling references
    // Dangling references occur when you have a pointer to a location in memory that may have been given to someone else, 
    // by freeing some memory while preserving a pointer to that memory.
    // This is a common bug in C programming. Rust ensures that this never happens: you cannot create a dangling reference in Rust.

    // let reference_to_nothing = dangle();

    // println!("{}", reference_to_nothing);

    // The solution is to return the String directly
    let dangle_s = no_dangle();

    println!("{}", dangle_s);

    // Slice type
    // A slice is a reference to a contiguous sequence of elements in a collection rather than the whole collection.
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{} {}", hello, world);

    first_word(&s);


}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special h

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn calculate_length_borrowing(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

/*fn dangle() -> &String {
    let s = String::from("hello");

    &s
} // s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
*/

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}