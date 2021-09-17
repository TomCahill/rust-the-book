use std::io;

fn print_title(title: &str) {
  println!("");
  println!("{}:", title);
  println!("======================");
}

fn main() {
  print_title("Integer Types:");

  println!("# 8-BIT:");
  println!("Unsigned: {}-{}", u8::MIN, u8::MAX);
  println!("Signed: {}-{}", i8::MIN, i8::MAX);

  println!("# 16-BIT:");
  println!("Unsigned: {}-{}", u16::MIN, u16::MAX);
  println!("Signed: {}-{}", i16::MIN, i16::MAX);

  println!("# 32-BIT:");
  println!("Unsigned: {}-{}", u32::MIN, u32::MAX);
  println!("Signed: {}-{}", i32::MIN, i32::MAX);

  println!("# 64-BIT:");
  println!("Unsigned: {}-{}", u64::MIN, u64::MAX);
  println!("Signed: {}-{}", i64::MIN, i64::MAX);

  println!("# 128-BIT:");
  println!("Unsigned: {}-{}", u128::MIN, u128::MAX);
  println!("Signed: {}-{}", i128::MIN, i128::MAX);

  println!("# Arch:");
  println!("Unsigned: {}-{}", usize::MIN, usize::MAX);
  println!("Signed: {}-{}", isize::MIN, isize::MAX);
  // println!("8-bit range signed: {}-{}", i8.MIN, i8.MAX)

  print_title("Integer Overflow");

  // Overflow wrapping (This should panic in debug mode but not release)
  let mut x: u8 = u8::MAX;
  println!("The next line should panic in debug mode but not in release...");
  println!("...");
  // Comment this line to allow building in debug mode
  // x = x + 1;
  println!("x should be equal to in release 0 after wrapping {} and not {}", x, u8::MAX);

  print_title("Floating-Point Types");
  let x = 2.0;
  println!("Default floating points types are 64bit {}", x);
  let y: f32 = 3.0;
  println!("but can be typed as 32 {}", y);

  print_title("Numeric Operations");

  println!("Addition: 5 + 10 = {}", 5 + 10);
  println!("Subtraction: 95.5 - 4.3 = {}", 95.5 - 4.3);
  println!("Multiplication: 4 * 30 = {}", 4 * 30);
  println!("Division: 56.7 / 32.2 = {}", 56.7 / 32.2);
  println!("Remainder: 43 % 5 = {}", 43 % 5);

  print_title("Boolean Type");
  let t = true;
  println!("t = {}", t);

  let f: bool = false; // with explicit type annotation
  println!("f = {}", f);
  println!("t == f = {}", t == f);

  print_title("The Character Type");
  let c = 'z';
  let z = 'ℤ';
  let heart_eyed_cat = '😻';
  println!("c: {} z: {} heart_eyed_cat: {}", c, z, heart_eyed_cat);

  print_title("Compound Types");

  print_title("The Tuple Type");
  let tup = (500, 6.4, 1);
  let (x, y, z) = tup;
  println!("The value of x,y,z destructured from tuple: {},{},{}", x, y, z);
  println!("The value of tup.1 accessed directly from tuple: {}", tup.1);

  print_title("The Array Type");
  let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
  println!("Accessing array element 4 directly from onths: {}", months[3]);

  println!("Please enter an array index to fetch the month, an index greater that 11 will result in a panic.");

  let mut index = String::new();

  io::stdin().read_line(&mut index).expect("Failed to read line");

  let index: usize = index
    .trim()
    .parse()
    .expect("Index entered was not a number");

    let element = months[index];

  println!(
    "The value of the element at index {} is: {}",
    index, element
  );
}
