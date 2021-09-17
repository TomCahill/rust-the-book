
const MAX_POINTS: u32 = 100_000;

fn main() {
  // shadowing
  let x = 5;
  println!("The value of x is: {}", x);
  let x = 6;
  println!("The value of x is: {}", x);

  // vs

  // mutable
  let mut y = 5;
  println!("The value of y is: {}", y);
  y = 6;
  println!("The value of y is: {}", y);

  println!("Max points are: {}", MAX_POINTS);

  let x = 5;
  println!("The value of x is: {}", x);

  let x = x + 1;
  println!("The value of x is: {}", x);

  let x = x * 2;
  println!("The value of x is: {}", x);

  let spaces = "   ";
  let spaces = spaces.len();
  println!("Number of spaces: {}", spaces);
}
