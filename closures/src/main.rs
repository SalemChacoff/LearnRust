use std::thread;
use std::time::Duration;

// Cacher struct is generic over type T
struct Cacher<T>
where T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where T: Fn(u32) -> u32,
{
    // Constructor for Cacher struct
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    // value method takes a u32 arg and returns a u32
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }

}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn main() {
    let simulated_intensity = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_intensity, simulated_random_number);

    // Clousures can capture their environment
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;

    // This not work because x is not in scope
    //fn equal_to_x(z: u32) -> bool { z == x }

    assert!(equal_to_x(y));

    // FnOnce consumes the variables it captures from its enclosing scope
    // FnMut can change the environment because it mutably borrows values
    // Fn borrows values from the environment immutably
    let x = vec![1, 2, 3];

    // If add move keyword, x is moved into closure, so x is not valid after this
    let equal_to_x = |z| z == x;

    // This not work because x is moved
    println!("can use x here: {:?}", x);

    let y = vec![1, 2, 3];
    
    assert!(equal_to_x(y));
}

fn generate_workout(intensity: u32, random_number: u32) {

    // Closure is lambda function
    let mut cached_result = Cacher::new(|num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", cached_result.value(intensity));
        println!("Next, do {} situps!", cached_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", cached_result.value(intensity));
        }
    }
}
