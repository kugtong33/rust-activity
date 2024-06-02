fn main() {
  // variables are immutablel by default, you can make them mutable by adding mut
  let mut x = 5;
  println!("The value of x is :{x}");
  x = 6;
  println!("The value of x is :{x}");

  // naming convention for constants is to use all uppercase with underscores
  // constants are valid for the entire time a program runs, within the scope in which they were declared
  const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

  println!("three hours in seconds is {THREE_HOURS_IN_SECONDS}");
}