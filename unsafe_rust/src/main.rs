
// Call a external C function
extern "C" {
    fn abs(input: i32) -> i32;
}


// Static variables, which are similar to constants, but static variables have a fixed memory address in the program's memory
static HELLO_WORLD: &str = "Hello, world!";

// Can be mutable, but unsafe
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

// Unsafe trait
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

fn main() {

    // Dereferencing a raw pointer
    let mut num = 5;
    
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // Unsafe keyword
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    //let address = 0x012345usize;
    //let r3 = address as *const i32;

    // Calling an unsafe function
    unsafe fn dangerous() {
        println!("This is a dangerous function");
    }

    unsafe {
        dangerous();
    }


    // Creating a safe abstraction over unsafe code

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // Extern Functions to call external code
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // Accessing or modifying a mutable static variable
    println!("name is: {}", HELLO_WORLD);

    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }



}


fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (std::slice::from_raw_parts_mut(ptr, mid),
         std::slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
    }

}

// Calling Rust from other languages
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}