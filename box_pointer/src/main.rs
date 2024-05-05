
// List is a recursive enum that can be used to create a linked list
// Need to use Box to store the data on the heap
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    // Box is a smart pointer that points to data on the heap
    let b = Box::new(5);
    println!("b = {}", b);

    // Creating a linked list using Box
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // loop through the list and print the values
    let mut current = &list;
    loop {
        match current {
            Cons(value, next) => {
                println!("Value: {}", value);
                current = next;
            }
            Nil => {
                println!("End of list");
                break;
            }
        }
    }

}
