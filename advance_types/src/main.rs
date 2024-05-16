use std::fmt::{self};

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

struct Age(u32);

struct ID(u32);

fn main() {

    // Newtype pattern
    let w = Wrapper(
        vec![String::from("hello"), String::from("world")]
    );

    println!("w = {}", w);

    // Type Aliases
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    // Never type, !, is a type that never returns, every function with ! is a diverging function
    fn bar() -> ! {
        panic!("This is a never type");
    }

    // Dynamically Sized Types and the Sized Trait
    // Dynamic Sized Types, DSTs, are types whose size can only be known at runtime
    let s1: &str = "Hello there!";
    let s2: &str = "How's it going?";

}

fn type_aliases() {
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_long_type(f: Thunk) {
        // --snip--
    }

    fn returns_long_type() -> Thunk {
        // --snip--
        Box::new(|| println!("hi"))
    }
}

fn generic<T: Sized>(t: T) {
    // --snip--
}