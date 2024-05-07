use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Implement Deref trait for MyBox
impl<T> Deref for MyBox<T> {
    // Define associated type Target
    type Target = T;

    // Implement deref method, which returns a reference to the value
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

// The Drop is used to run some code when a value goes out of scope
// This cleanup automatically happens at the end of the block
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    // Deref
    let x = 5;
    // y is a reference to x
    let y = MyBox::new(x);

    assert_eq!(5, x);
    // *y is the value of x
    assert_eq!(5, *y);
    // This code works because of deref coercion, Rust can convert &MyBox<i32> to &i32
    // This code is equivalent to *y
    assert_eq!(5, *(y.deref()));

    // This not work because y is a reference
    //assert_eq!(5, y);

    let m = MyBox::new(String::from("Rust"));
    // Pass a reference to a value of MyBox
    hello(&m);

    // This code works because
    // (*m) derefer and returns the value of String
    // [..] is a slice of the entire string
    // &(*m)[..] is a reference to the slice
    hello(&(*m)[..]);

    // Rust does deref coercion when it finds types and trait implementations in three cases:
    // 1. From &T to &U when T: Deref<Target=U>
    // 2. From &mut T to &mut U when T: DerefMut<Target=U>
    // 3. From &mut T to &U when T: Deref<Target=U>

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
    
    // This not work because rust removes the value from the scope automatically
    //c.drop();

    // If you want to force the drop of a value, you can use std::mem::drop
    drop(c);
    println!("CustomSmartPointer c dropped before the end of the block.");

}

fn hello(name: &str) {
    println!("Hello, {}!", name)
}
