use std::sync::{Arc, Mutex};
use std::thread;
use std::rc::Rc;

fn main() {

    // Mutex is a mutual exclusion primitive useful for protecting shared data
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);

    sharing_mutex_between_multiple_threads();
}

fn sharing_mutex_between_multiple_threads() {

    // Arc is an atomically reference counted type
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move | | {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
