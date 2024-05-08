use std::sync::mpsc;
use std::thread;

fn main() {

    let (tx, rx) = mpsc::channel();

    thread::spawn(move | | {
        let msg = String::from("hi");
        // When you pass a value to the send method, you are transferring ownership of the value to the receiving end of the channel
        tx.send(msg).unwrap();
        // You can't use msg here because it has been moved to the receiving end of the channel
        // println!("msg: {}", msg);
    });

    // recv blocks the main thread until a value is sent down the channel
    // try_recv does not block the main thread, if there is no value sent down the channel, it returns an error
    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    println!("Pass multiple values");
    pass_multiple_values();

    println!("Creating multiple producers");
    creating_multiple_producers();

}

fn pass_multiple_values() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move | | {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(std::time::Duration::from_secs(1));
            // You can't use val here because it has been moved to the receiving end of the channel
            // println!("val: {}", val);
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

fn creating_multiple_producers() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move | | {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    thread::spawn(move | | {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
