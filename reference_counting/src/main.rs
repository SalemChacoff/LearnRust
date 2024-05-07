use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {

    // Rc<T> allows multiple ownership of the same data
    // Rc<T> keeps track of the number of references to a value
    // Rc<T> only deals with immutable references

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, a.clone());
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, a.clone());
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
