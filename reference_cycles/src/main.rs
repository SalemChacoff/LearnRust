use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

// RC<T> allows multiple ownership of the same data
// RefCell<T> allows mutable borrows checked at runtime

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));

    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        // This will create a cycle a -> b -> a
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle
    // it will overflow the stack
    //println!("a next item = {:?}", a.tail());

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    
    // This will create a cycle leaf -> branch -> leaf
    *leaf.children.borrow_mut() = vec![Rc::clone(&branch)];

    // Uncomment the next line to see that we have a cycle
    // it will overflow the stack
    //println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());


    
    // Strong count and weak count
    // Strong count is the number of reference which have ownership of the value
    // Weak count is the number of reference which do not have ownership of the value

    let leaf_parent = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf_parent),
        Rc::weak_count(&leaf_parent)
    );

    {
        let branch_parent = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf_parent)]),
        });

        *leaf_parent.parent.borrow_mut() = Rc::downgrade(&branch_parent);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch_parent),
            Rc::weak_count(&branch_parent)
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf_parent),
            Rc::weak_count(&leaf_parent)
        );
    }
    

}


#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}
