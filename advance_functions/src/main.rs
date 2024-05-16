
fn add_one(x: i32) -> i32 {
    x + 1
}

// Pass a function as an argument
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

// Fn, FnMut, and FnOnce are traits that are implemented by all closures.


fn main() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        .map(ToString::to_string)
        .collect();

    println!("{:?}", list_of_strings);


    enum Status {
        Value(u32),
        Stop,
    }

    // The type annotation for the closure is Fn(i32) -> i32
    let list_of_statuses: Vec<Status> = (0u32..20)
        .map(Status::Value)
        .collect();

    //println!("{:?}", list_of_statuses);

    // Returning Closures
    let closure = return_closure(1);

    let result = closure(5);
    println!("The result is: {}", result);
}

fn return_closure(a: i32) -> Box<dyn Fn(i32) -> i32> {
    if a > 0 {
        Box::new(move |b| a + b)
    } else {
        Box::new(move |b| a - b)
    }
}
