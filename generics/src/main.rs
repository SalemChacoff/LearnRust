
// Struct with generic type
struct Point<T, U> {
    x: T,
    y: U,
}

// Enum with generic type
#[derive(Debug)]
enum Option<T> {
    Some(T),
    None,
}

// Enum with multiple generic types
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// Generics In Method Definitions
impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    // Generic function
    let number_list = vec![34, 50, 25, 100, 65];
    let largest = get_largest(number_list);
    println!("The largest number is {}", largest);
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let largest = get_largest(number_list);
    println!("The largest number is {}", largest);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let largest = get_largest(char_list);
    println!("The largest char is {}", largest);

    // Generic struct
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let p3 = Point { x: 5, y: 10.0 };

    println!("x = {}, y = {}", integer.x, integer.y);
    println!("x = {}, y = {}", float.x, float.y);
    println!("x = {}, y = {}", p3.x, p3.y);

    // Generic enum
    let some_integer = Option::Some(5);
    let some_float = Option::Some(5.0);

    println!("{:?}", some_integer);
    println!("{:?}", some_float);

    // Generics In Method Definitions
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);


}

// Generic function
fn get_largest<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn get_largest_number(number_list: Vec<i32>) -> i32 {
    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn get_largest_char(number_list: Vec<char>) -> char {
    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}
