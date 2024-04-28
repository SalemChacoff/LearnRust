use unicode_segmentation::UnicodeSegmentation;
use std::collections::HashMap;

fn main() {

    // Creating a new vector
    let a: [i32; 3] = [1, 2, 3];
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);

    // Creating a vector with initial values
    {
        let v2 = vec![1, 2, 3];
    }


    // Accessing elements of a vector
    let third = &v[2];
    println!("The third element is {}", third);

    // Accessing elements of a vector with get, manage out of bounds
    match v.get(20) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // Iterating over the values in a vector
    for i in &v {
        println!("{}", i);
    }

    // Iterating over the values in a vector and changing them
    for i in &mut v {
        *i += 50;
    }

    // Showing the new values
    for i in &v {
        println!("{}", i);
    }

    // Using an enum to store multiple types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    
    
    match &row[1] {
        SpreadsheetCell::Int(i) => println!("Int: {}", i),
        _ => println!("Not an Int"),
    }

    // Using a string to store text
    // Strings are stored as a collection of UTF-8 encoded bytes
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");

    // Concatenating strings
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("{}", s3);

    // Using format! to concatenate strings
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);

    println!("{}", s);
    
    // Indexing into strings
    let s1 = String::from("hello");
    // let h = s1[0]; // This will not work
    let h = &s1[0..1];

    // Iterating over strings

    // Interating by bytes
    for c in "नमस्ते".bytes() {
        println!("{}", c);
    }

    println!("");

    // Interating by characters
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    println!("");

    // Interating by grapheme clusters
    for c in "नमस्ते".graphemes(true) {
        println!("{}", c);
    }

    // Using a hash map to store a set of key-value pairs
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");
    let mut scores = HashMap::new();

    scores.insert(blue, 10);
    scores.insert(yellow, 50);

    // Accessing values in a hash map
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    match score {
        Some(s) => println!("The score for {} is {}", team_name, s),
        None => println!("No score for {}", team_name),
    }

    // Overwriting a value in a hash map
    scores.insert(String::from("Blue"), 25);

    // Interating over a hash map
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let text = "Hellow world wonderful world";
    let mut map = HashMap::new();

    // Split the text into words and count the number of times each word is used
    // ["Hellow", "world", "wonderful", "world"]
    for word in text.split_whitespace() {
        // Get the value for the key word, if it exists, or insert 0
        let count = map.entry(word).or_insert(0);
        // Increment the value for the key word
        *count += 1;
    }

    println!("{:?}", map);

}
