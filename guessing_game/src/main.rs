use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        // Variables in Rust are immutable by default.
        // To make them mutable, you need to add the mut keyword.
        let mut guess = String::new();

        // & indicates that this argument is a reference,
        // which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Shadowing is a useful feature in Rust that allows you to reuse the variable name.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        println!("You guessed: {guess}");

        // The match expression is made up of arms you can assign values to.
        // The match expression takes an expression and then branches into code based on the resulting value.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    
    /*let x = 5;
    let y = 10;

    println!("x = {x} and y + 2 = {}", y + 2);*/
}
