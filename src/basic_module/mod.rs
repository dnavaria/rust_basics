// #[allow(dead_code)]
pub fn basic() {
    // mutable variable 
    let x: i32 = 10;
    println!("{}", x);
    let x = "20";
    println!("{}", x);

    // constant variable
    const MAX_USER: u32 = 100_000;
    println!("{}", MAX_USER);
    
    // Scalar Data types
    // Integer
    let x: i32 = 10;
    println!("{}", x);

    // Floating-point numbers
    let x: f64 = 10.0;
    println!("{}", x);

    // Boolean
    let x: bool = true;
    println!("{}", x);

    // Character
    let x: char = 'a';
    println!("{}", x);

    // Compound Data types
    // Tuple
    let tup: (i32, f64, bool) = (10, 10.0, true);

    // Destructuring
    let (x, y, z) = tup;
    println!("Destructuring => {} {} {}", x, y, z);

    // Accessing tuple element
    println!("Accessing tuple element => {} {} {}", tup.0, tup.1, tup.2);

    // Array, fixed length. All elements must be of same type.
    // For dynamic length, use vector.
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Accessing array element => {}", arr[4]);

    let byte = [0; 8];
    println!("Accessing array element => {}", byte[7]);

    // Function, last expression is implicitly returned
    let result: i32 = another_function(11, 12);
    println!("Result => {}", result);

    // If expression
    if result == 23 {
        println!("Result is 23");
    } else if result < 23 {
        println!("Result less than 23");
    }else {
        println!("Result greater than 23");
    }

    let number: i32 = if result == 32 { 5 } else { 6 };
    println!("Number => {}", number);

    // Loop
    let mut counter: i32 = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Loop result => {}", result);

    // While loop
    let mut number: i32 = 3;
    while number != 0 {
        println!("While loop => {}", number);
        number -= 1;
    }

    // For loop
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    for element in arr.iter() {
        println!("For loop => {}", element);
    }

    // Ownership Rules
    // 1. Each value in Rust has a variable that’s called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.
    {
        let s = String::from("hello"); // s is valid from this point forward
        println!("{}", s);
     // this scope is now over, and s is no longer valid
    }

    // Move
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}", s1); // error: value borrowed here after move
    println!("{}", s2); // valid

    // Clone
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2); // valid

    // Copy
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y); // valid

    // Ownership and Functions
    let s = String::from("hello"); // s comes into scope
    takes_ownership(s); // s's value moves into the function...
    // println!("{}", s); // error: value borrowed here after move

    let x = 5; // x comes into scope
    makes_copy(x); // x would move into the function,
    println!("{}", x); // but i32 is Copy, so it’s okay to still use x afterward
    
    // Take ownership and return ownership
    let s1: String = gives_ownership(); // gives_ownership moves its return
    let s2: String = String::from("hello"); // s2 comes into scope
    let s3: String = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, which also moves its return value into s3
    println!("{},{}",  s1, s3);

    // References and Borrowing
    let s1: String = String::from("hello");
    let len: usize = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // Mutable References
    // We can have multiple immutable references, but only one mutable reference
    // We can’t have a mutable reference while we have an immutable one
    // References must always be valid, that means we can’t have a reference to a value that doesn’t exist
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);

    // Dangling References
    // let reference_to_nothing = dangle();
    // println!("{}", reference_to_nothing);

    // Slices
    let s = String::from("hello world");
    let hello: &str = &s[0..5];
    let world: &str = &s[6..11];
    println!("{} {}", hello, world);

    let result = extract_first_word_length(&s);
    println!("{}", result);

}

fn extract_first_word_length(word: &String) -> &str {
    let bytes = word.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &word[0..i];
        }
    }
    &word[..]
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

fn another_function(x: i32, y: i32) -> i32 {
    println!("Another function => {} {}", x, y); 
    x+y
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(x: i32) { // x comes into scope
    println!("{}", x);
} 

fn gives_ownership() -> String{
    let some_string = String::from("hello"); // some_string comes into scope
    some_string // some_string is returned and moves out to the calling function
}

fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: &String) -> usize{
    s.len()
}

fn change(s: &mut String){
    s.push_str(", world");
}

