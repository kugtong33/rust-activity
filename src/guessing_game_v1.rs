// the prelude is the list of things that Rust automatically imports into every Rust program.
// preludes can be seen as a pattern to make using multiple types more convenient
use std::io;

// fn syntax declares a function
// () indicates there are no parameters
// {} is the body of the function
fn main() {
    println!("Guess the number!");

    println!("Please input your guess");

    // in Rust, variables are immutable by default
    // once we give variable a value, that value won't change
    // to make a variable mutable, we add mut before the variable name
    let mut guess = String::new();

    io::stdin()
        // the string argument needs to be mutable so the method can change the string's content
        // the & indicates that this argument is a reference
        // which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times
        // Result is an enumeration, which is a type that can be in one of multiple possible states
        .read_line(&mut guess)
        // an instance of Result has an expect method that you can call
        // If Result is an Err, expect() will cause the program to crash and display the message
        // if Result is an Ok value, expect() will take the return value that Ok is holding 
        .expect("failed to read line");

    println!("you guessed: {guess}");
}