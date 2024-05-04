
// The Iterator trait is defined in the standard library. Here is a simplified version of its definition:
/*pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}*/

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

// Struct To Create Custom Iterators
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for value in v1_iter {
        println!("Got: {}", value);
    }

    // Map method take a closure as an argument and calls that closure on each item to produce a new iterator.
    let v1: Vec<i32> = vec![1, 2, 3];
    let result: Vec<i32> = v1.iter().map(|x| return x + 1).collect();
    println!("{:?}", result);
    assert!(result == vec![2, 3, 4]);

    // Filter method takes a closure that takes each item from the iterator and returns a Boolean.
    // If the closure returns true, the value is included in the iterator produced by filter.
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];
    let in_my_size = shoes_in_my_size(shoes, 10);
    println!("{:?}", in_my_size);
    

}


#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    // into_iter method produces an iterator that takes ownership of the collection and returns owned values.
    let mut v1_iter = v1.into_iter();
    
    // iter_mut method produces an iterator that borrows each element of the collection through each iteration.
    //let mut v1_iter = v1.iter_mut();

    // iter method need to access the collection in a read-only way.
    //let mut v1_iter = v1.iter();

    


    // Some is a variant of the Option enum, which is returned by the next method of the Iterator trait.
    // None is another variant of the Option enum, which is returned by the next method of the Iterator trait.
    assert_eq!(v1_iter.next(), Some(1));
    assert_eq!(v1_iter.next(), Some(2));
    assert_eq!(v1_iter.next(), Some(3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    // into_iter method produces an iterator that takes ownership of the collection and returns owned values.
    let v1_iter = v1.into_iter();

    // sum method is a consuming adapter, because it takes ownership of the iterator it is called on.
    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}

// Use custom iterator
#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();

    assert_eq!(sum, 18);
}