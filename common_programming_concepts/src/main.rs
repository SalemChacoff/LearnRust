fn main() {
    // Variables and Mutability
    // keyword let is used to create a variable
    // variable is immutable by default
    // to make it mutable, use keyword mut
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Constants
    // Constants are always immutable
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");

    // Shadowing
    // Shadowing overrides the previous value of a variable
    let y = 5;
    let y = y + 1;

    // The value change in the inner scope does not affect the value in the outer scope
    {
        let y = y * 2;
        println!("The value of y in inner scope is: {y}");
    }

    println!("The value of y is: {y}");


    // The following code will not compile
    // Cannot change the type of a variable
    /*let mut spaces = "   ";
    spaces = spaces.len();
    println!("The value of spaces is: {spaces}");*/

    // The following code will compile
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}");

    // Data Types
    // Rust is a statically typed language
    // Rust is a strongly typed language
    // Rust has type inference
    let _guess: u32 = "42".parse().expect("Not a number!");

    // Scalar Types
    // Integer Types
    // Length	Signed	Unsigned
    // 8-bit	i8	    u8
    // 16-bit	i16	    u16
    // 32-bit	i32	    u32
    // 64-bit	i64	    u64
    // 128-bit	i128	u128
    // arch	isize	usize
    let _x: i8 = 127;
    let _y: u8 = 255;

    println!("The value of x is: {_x}");
    println!("The value of y is: {_y}");

    // Floating-Point Types
    // Rust has two primitive types for floating-point numbers: f32 and f64
    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32

    println!("The value of x is: {_x}");
    println!("The value of y is: {_y}");

    // Numeric Operations
    // Addition
    let sum = 5 + 10;
    println!("The value of sum is: {sum}");
    // Subtraction
    let difference = 95.5 - 4.3;
    println!("The value of difference is: {difference}");
    // Multiplication
    let product = 4 * 30;
    println!("The value of product is: {product}");
    // Division
    let quotient = 56.7 / 32.2;
    println!("The value of quotient is: {quotient}");
    // Remainder
    let remainder = 43 % 5;
    println!("The value of remainder is: {remainder}");

    // Boolean Type
    let _t = true;
    let _f: bool = false;

    println!("The value of t is: {_t}");
    println!("The value of f is: {_f}");

    // Character Type
    let _c = 'z';
    let _z = 'ℤ';
    let _heart_eyed_cat = '😻';

    println!("The value of c is: {_c}");
    println!("The value of z is: {_z}");
    println!("The value of heart_eyed_cat is: {_heart_eyed_cat}");

    // Compound Types
    // Tuple Type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, _y, _z) = tup;
    println!("The value of y is: {_y}");

    // Accessing tuple elements using dot notation
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("The value of five_hundred is: {five_hundred}");
    println!("The value of six_point_four is: {six_point_four}");
    println!("The value of one is: {one}");

    // Array Type
    let _a = [1, 2, 3, 4, 5];
    let _months = ["January", "February", "March", "April", "May", "June", "July",
                   "August", "September", "October", "November", "December"];

    // Accessing array elements
    let _first = _a[0];
    let _second = _a[1];

    println!("The value of first is: {_first}");
    println!("The value of second is: {_second}");

    // Accessing array elements using index
    let _index = 10;
    let _element = _months[_index];
    println!("The value of element is: {_element}");

    // The following code will not compile
    // Array index out of bounds
    /*let _index = 10;
    let _element = _a[_index];
    println!("The value of element is: {_element}");*/

    // Functions
    another_function(5);

    // Function with parameters
    print_labeled_measurement(5, 'm');

    // Function with return value
    let _five:i32 = 5;
    let _six:i32 = plus_one(5);
    println!("The value of six is: {_six}");

    // Control Flow
    // if expression
    let number = 3;
    if number < 5 {
        println!("The condition is true");
    } else {
        println!("The condition is false");
    }

    // if expression with else if
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Using if in a let statement
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    // Repetition with Loops
    // loop keyword
    loop {
        println!("again!");
        break;
    }

    // Returning a value from a loop
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The value of result is: {result}");

    // Conditional Loops with while
    let mut number = 3;
    while number != 0 {
        println!("{number}");

        number -= 1;
    }

}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn plus_one(x: i32) -> i32 {
    return x + 1;
}