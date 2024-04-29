use std::fmt::Display;


struct ImportantExcept<'a> {
    part: &'a str,
}

// Lifetime annotations in function signatures
// We need to annotate lifetimes in function signatures when we have references as parameters or return values
// The names of lifetime parameters must start with an apostrophe (') and are usually all lowercase and very short, like generic types
impl<'a> ImportantExcept<'a> {
    fn return_part(&'a self, announcement: &str) -> &'a str {
        println!("Attentions please: {}", announcement);
        self.part
    }
}

// Lifetime annotations in function signatures
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

fn main() {
    let string1 = String::from("abcd");
    let result;
    
    {
        let string2 = String::from("xyz"); // string2 is not valid after this block
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    // string2 is not valid here, beacuse it's scope is limited to the above block
    //println!("The longest string is {}", result);

    // Lifetime annotations in struct definitions
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcept { part: first_sentence };


    // Lifetime Elision
    let s: &str = "Hello world!";
    let first = first_word(s);
    println!("The first word is: {}", first);

    // Static lifetime
    let s: &'static str = "I have a static lifetime.";
    println!("{}", s);

    // Lifetime annotations in function signatures
    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");
    let ann = "Announcement!";
    let result = longest_with_an_announcement(string1.as_str(), string2.as_str(), ann);
    println!("The longest string is {}", result);

}


// &i32    // a reference
// &'a i32 // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 1. Each parameter that is a reference gets its own lifetime parameter.

// 2. If there is exactly one input lifetime parameter, 
// that lifetime is assigned to all output lifetime parameters.

// 3. If there are multiple input lifetime parameters, 
// but one of them is &self or &mut self because this is a method, 
// the lifetime of self is assigned to all output lifetime parameters.

fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();

    for (i , &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}