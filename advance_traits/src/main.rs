use core::fmt;
use std::ops::Add;

pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}

// Associated type:
// - It is a type that is associated with a trait.
// - It is a way to define a placeholder type that will be used in the implementation of the trait methods.
// - It is a way to define a type that is used in the trait methods, but is not known until the trait is implemented.

struct Counter {}

impl Iterator<u32> for Counter {
    fn next(&mut self) -> Option<u32> {
        Some(0)
    }
}

impl Iterator<u16> for Counter {

    fn next(&mut self) -> Option<u16> {
        Some(0)
    }
}

// Default generic type Parameter:

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Calling Methods with the Same Name:

trait Pilot { 
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

// Supertraits:

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    <Human as Pilot>::fly(&person);


    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);

}
