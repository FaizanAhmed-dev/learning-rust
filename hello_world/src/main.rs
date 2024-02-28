// Declare the module
mod date;

// Import the function
use date::get_date;
use date::current_timestamp;
use date::format_timestamp;

// Pattern Matching:
// Pattern matching allows you to match different patterns and execute corresponding code blocks.
fn pattern_matching() {
    let number = 7;

    match number {
        1 => println!("It's one"),
        2 | 3 => println!("It's two or three"),
        4..=7 => println!("It's between 4 and 7"),
        _ => println!("It's something else"),
    }
}

fn variables_and_types() {
    // Variables and Data Types:
    // In Rust, you declare variables using the let keyword. Rust is statically typed, which means that the compiler must know the types of all variables at compile time.
    let age: i32 = 25; // Declare an integer variable
    let pi: f64 = 3.14159; // Declare a floating-point variable
    let is_rust_fun: bool = true; // Declare a boolean variable
    let first_initial: char = 'J'; // Declare a character variable

    println!("Age: {}", age);
    println!("Pi: {}", pi);
    println!("Is Rust fun? {}", is_rust_fun);
    println!("First Initial: {}", first_initial);
}

fn conditions() {
    let is_sunny = true;
    let is_warm = true;

    // Using && (logical AND)
    if is_sunny && is_warm {
        println!("It's a perfect day!");
    } else {
        println!("It's not a perfect day.");
    }

    let is_raining = false;

    // Using || (logical OR)
    if is_sunny || is_raining {
        println!("You might want to carry an umbrella.");
    } else {
        println!("No need for an umbrella.");
    }

    let is_cloudy = true;

    // Using ! (logical NOT)
    if !is_cloudy {
        println!("The sky is clear.");
    } else {
        println!("It's cloudy outside.");
    }
}

fn loops() {
    // #loop Loop:
    // The loop keyword starts an infinite loop. You can use this loop when you want to repeatedly execute a block of code until a specific condition is met. To exit the loop, you usually use the break keyword.
    loop {
        // Code that runs indefinitely unless there's a "break"
        println!("This is an infinite loop.");
        break; // Exit the loop
    }

    // #while Loop:
    // The while loop continues to execute as long as a given condition is true. It checks the condition before each iteration.
    let mut count = 0;

    while count < 5 {
        println!("Count: {}", count);
        count += 1;
    }

    // #for Loop:
    // The for loop is used to iterate over a range or collection, such as an array or a range of numbers.
    for number in 1..=5 {
        println!("Number: {}", number);
    }

    let fruits = ["apple", "banana", "cherry"];

    for fruit in &fruits {
        println!("Fruit: {}", fruit);
    }

    // #for_each Method:
    // You can use the .iter() method along with .for_each() to iterate through elements in a collection.
    let fruits = ["apple", "banana", "cherry"];

    fruits.iter().for_each(|fruit| {
        println!("Fruit: {}", fruit);
    });

    // #loop with break for User Input:
    // A common use of the loop construct is for reading user input until a specific condition is met.
    loop {
        println!("Enter 'q' to quit:");

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim() == "q" {
            break;
        }
    }
}

// Functions and Methods:
// Functions in Rust are declared using the fn keyword. They can take parameters and return values. Rust's return value is the result of the last expression in the function, which means you don't need an explicit return keyword.
fn add(a: i32, b: i32) -> i32 {
    a + b // This is the return value
}

// Methods in Rust are associated functions that are called on instances of structs or enums using the dot notation.
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// To borrow data without transferring ownership, you can use references:
fn calculate_length(s: &String) -> usize {
    s.len()
}

// Structs and Enums:
// Structs are used to create custom data types with named fields.
struct Person {
    name: String,
    age: u32,
}
// Enums are used to define a type that can have different values.

// Structs and Enums:
// Structs are used to create custom data types with named fields.
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

// Enums are used to define a type that can have different values.
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    variables_and_types();
    pattern_matching();
    conditions();
    loops();
    get_date();

    let result: i32 = add(5, 3);
    println!("Result: {}", result);

    let rect = Rectangle {
        width: 10,
        height: 20,
    };
    println!("Area: {}", rect.area());

    // @Ownership, Borrowing, and Lifetimes:
    // Rust's ownership system ensures memory safety by tracking how data is shared and managed. Variables have ownership, and you can transfer ownership using functions or use borrowing to temporarily access data without transferring ownership.
    let s1 = String::from("hello"); // s1 owns the string

    let s2 = s1; // Ownership is moved to s2

    // The next line would cause a compile error since s1 no longer owns the string
    // println!("s1: {}", s1);

    println!("s2: {}", s2); // s2 still owns the string

    // To borrow data without transferring ownership, you can use references:
    // Lifetimes specify how long references are valid and prevent dangling references.
    let s: String = String::from("hello");
    let len = calculate_length(&s); // Borrowing s
    println!("Length: {}", len);

    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    println!("Name: {}, Age: {}", person.name, person.age);

    let coin_penny = Coin::Penny;
    let coin_dime = Coin::Dime;
    let coin_nickel = Coin::Nickel;
    let coin_quarter: Coin = Coin::Quarter;

    value_in_cents(coin_dime);
    value_in_cents(coin_nickel);
    value_in_cents(coin_quarter);

    let value = value_in_cents(coin_penny);
    println!("Value: {} cents", value);
}
