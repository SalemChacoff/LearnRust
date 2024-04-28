
// Enum definition
enum IpAddrKind {
    V4,
    V6,
}

enum IpAddrKindWithValues {
    V4(String),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum Message {
    Quit, // No data associated
    Move { x: i32, y: i32 }, // has named fields, like a struct does
    Write(String), // includes a single String
    ChangeColor(i32, i32, i32), // includes three i32 values
}

struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}

struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct


impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

enum Option<T> {
    None,
    Some(T),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

fn main() {

    // Enum usage
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    // Struct usage
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("{}", home.address);
    println!("{}", loopback.address);

    // Enum with values
    let home = IpAddrKindWithValues::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKindWithValues::V6(String::from("::1"));

    // Enum with values usage
    let m = Message::Write(String::from("hello"));
    m.call();
    
    // Option enum usage
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = Option::None;
    
    /*let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y; */

    // Coin enum usage
    let coin = Coin::Penny;
    let value = value_in_cents(coin);
    println!("{}", value);

    // plus_one function usage
    let five = Some(5);
    //let six = plus_one(five);
    let none = plus_one(Option::None);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}

    
    // if let syntax
    let config_max = Some(3u8);
    match config_max {
        Some(max) => {
            println!("The maximum is configured to be {}", max);
        },
        _ => (),
    }




}

fn route(ip_kind: IpAddrKind) {}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Option::None => Option::None,
        Option::Some(i) => Option::Some(i + 1),
    }
}