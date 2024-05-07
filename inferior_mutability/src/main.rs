
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::{cell::RefCell, rc::Rc};


fn main() {

    // Rc<T> allows multiple ownership of the same data; Box<T> and RefCell<T> have single ownership.
    let value = Rc::new(RefCell::new(5));

    // Rc::clone only increments the reference count, which doesn't create a deep copy of the data.
    let a = Rc::new(List::Cons(Rc::clone(&value), Rc::new(List::Nil)));

    // Rc::clone doesn't make a deep copy of the data, so the data inside the RefCell<i32> in a is the same data inside value.
    let b = List::Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));

    // c is a Cons variant that has a reference to the same RefCell<i32> that a does.
    let c = List::Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    // We can modify the value inside the RefCell<i32> in a through the reference in value.
    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
