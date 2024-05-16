use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    let v1: Vec<u32> = vec![1, 2, 3];
    let v2: Vec<&str> = vec!["a", "b", "c", "d", "e"];

    Pancakes::hello_macro();
}

// Attributes like macros
/*#[route(GET, "/")]
fn index() {
    // code
}

#[proc_macro_attribute]
pub fn route(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    // code
}*/


// Function-like macros
/*let sql = sql!(SELECT * FROM posts WHERE id=1);*/
/*#[proc_macro]
pub fn sql(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // code
}*/


