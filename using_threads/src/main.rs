use std::{thread, time::Duration};

fn main() {
    // Rust implements one-to-one threading model
    // This means that each thread is mapped to an OS thread

    let thread1 = thread::spawn(| | {
        for i in 1..10 {
            println!("hi number {} from the spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // The main thread does not wait for the spawned thread to finish
    for i in 1..5 {
        println!("hi number {} from the main thread", i);
        thread::sleep(Duration::from_millis(1));
    };

    // We can use the join method to wait for the spawned thread to finish
    thread1.join().unwrap();

    let v = vec![1, 2, 3];

    // We can move ownership of variables to the spawned thread
    let handle = thread::spawn(move | | {
        println!("Here's a vector: {:?}", v);
    });

    // We can't use v here because it has been moved to the spawned thread
    //drop(v); // This will cause a compilation error if we try to use v after this line

    handle.join().unwrap();

}
