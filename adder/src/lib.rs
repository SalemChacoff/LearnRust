pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[derive(Debug)]
struct Rectangule {
    width: u32,
    height: u32,
}

impl Rectangule {
    fn can_hold(&self, other: &Rectangule) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        else if value > 100 {
            panic!("Guess value is too high, must be between 1 and 100, got {}.", value);
        }
        Guess {
            value
        }
    }
}

fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod tests {
    // super:: is the parent module of the current module
    // Path: adder/src/lib.rs
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangule {
            width: 8,
            height: 7,
        };
        let smaller = Rectangule {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    
    #[test]
    fn smaller_cannot_hold_smaller() {
        let larger = Rectangule {
            width: 8,
            height: 7,
        };
        let smaller = Rectangule {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn it_adds_two_fail() {
        assert_ne!(5, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }

    // Custom error message
    // should_panic is used to test that a function panics
    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100")]
    fn greater_than_100() {
        Guess::new(-2);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("Two plus two does not equal four"))
        }
    }

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }

    /* 
    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }*/

    /* 
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn failing_test() {
        panic!("Make this test fail");
    } */
}
