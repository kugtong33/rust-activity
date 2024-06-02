use std::{cmp::Ordering, io};
// The Rng trait defines methods that random number generators implement, and this trait must be in scope for us to use those methods.
// 
use rand::Rng;


fn main() {
  println!("Guess the number");

  // we call the rand::thread_rng function that gives us the particular random number generator we're going to use: one that is local to the current thread of execution and is seeded by the operating system
  // the gen_range() method takes a range expression as an argument and generates a random number in the range
  // the kind of range expression we're using here takes the form start..=end
  let secret_number = rand::thread_rng().gen_range(1..=100);

  println!("The secret number is: {secret_number}");

  // the loop keyword creates an infinite loop
  loop {
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line");

    // shadowing lets us reuse the guess variable name rather than forcig us to create two unique variables
    // bind this new variable to the expression guess.trim().parse()
    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {guess}");

    // given 50 as input and 38 as generated
    // match expression gets the Ordering::Greater, 50 > 38
    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => {
        println!("You win!");
        break;
      }
    }
  }
}