fn main() {
  let x = 5;

  // the first variable is shadowed by the second
  // which means that the second variable is what the compiler will see when you use the name of the variable
  // the second variable overshadowed the first, taking any uses of the variable name to itself
  let x = x + 1;

  {
      let x = x + 2;
      println!("The value of x in the inner scope is :{x}");
  }

  println!("The value of x is :{x}");

  let spaces = " ";
  let spaces = spaces.len();

  print!("Spaces length is :{spaces}");

  // if we try to use mut for this, as shown here, we’ll get a compile-time error:
  // let mut spaces = " ";
  // spaces = spaces.len();
}