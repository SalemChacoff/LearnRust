fn main() {

    // ==============================
    //      Match Expressions
    // ==============================

    #[derive(Debug)]
    enum Language {
        English,
        Spanish,
        Russian,
        Japanese,
    }

    let language = Language::English;

    match language {
        Language::English => println!("Hello World!"),
        Language::Spanish => println!("Hola Mundo!"),
        Language::Russian => println!("Привет мир!"),
        Language::Japanese => println!("こんにちは世界!"),
        //_ => println!("Language not supported!"),
        // You cand use the _ to match any other value o the enum self
        lang => println!("Language not supported: {:?}", lang),
    }

    // =======================================
    //      Conditional if let Expressions
    // =======================================

    let authorization_status: Option<&str> = None;
    let is_admin = false;
    let group_id: Result<u8, _> = "34".parse();

    if let Some(status) = authorization_status {
        println!("Authorization status: {}", status);
    } else if is_admin {
        println!("User is an admin");
    } else if let Ok(group_id) = group_id {
        if group_id > 30 {
            println!("Authorization status: privileged")
        } else {
            println!("Authorization status: basic")
        }
    } else {
        println!("Authorization status: guest");
    }

    // =======================================
    //     while let Conditional Loops
    // =======================================

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // =======================================
    //             for Loops
    // =======================================

    let v = vec!['a', 'b', 'c', 'd', 'e'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // =======================================
    //             let Statements
    // =======================================

    let x = 5;

    // let PATTERN = EXPRESSION;

    let (x,y,z) = (1,2,3);

    // =======================================
    //            Function Parameters
    // =======================================

    let point = (3, 5);
    print_coordinates(&point);

    // Irrustable
    let x = 5;

    // Refutable
    let x: Option<&str> = None;
    if let Some(x) = x {
        println!("{}", x);
    }

    // Can only accept irreductible patterns
    // function parameters
    // let statement
    // for loops

}


fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}