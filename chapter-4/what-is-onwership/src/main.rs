fn main() {
  // String is created on the heap
  let mut s = String::from("hello");
  s.push_str(", world!");

  println!("{}", s);

  // This value is pushed directly onto the stack as it's as small, well defined
  // and known at compile time.
  let x = 5;
  // The value is copied from x and also pushed onto the stack
  let y = x;
  println!("x = {}, y = {}", x, y);

  // Creates the value on the heap and a pointer on the stack
  let s1 = String::from("hello");
  // Creates a new pointer on the stack referencing the same value on the heap
  let s2 = s1;

  // NOTE: This will release the s1 pointer on the stack so we've effectively
  // moved s1 into s2. Referencing s1 will result in a complier error.
  // println!("{}, world!", s1);

  println!("{}, world!", s2);

  let s1 = String::from("hello");
  // Same scenario as above apart from we clone the value on the cheap and both
  // pointers are kept
  let s2 = s1.clone();

  println!("s1 = {}, s2 = {}", s1, s2);

  let s = String::from("Hello");
  takes_ownership(s);

  let x = 5;
  makes_copy(x);

  // Get a sting from a function
  let s1 = gives_ownership();
  let s2 = String::from("World");
  println!("s1 = {}, s2 = {}", s1, s2);
  let s3 = takes_and_gives_back(s2);
  // NOTE: we can't use s2 here because it's not valid after move to s3
  // println!("s1 = {}, s2 = {}, s2 = {}", s1, s2, s3);
  println!("s1 = {}, s2 = {}", s1, s3);

  let s1 = String::from("Hello");
  // using tuple's we can return multiple values from a function
  let (s1, len) = calculate_len(s1);

  println!("s1 = {}, len = {}", s1, len);
}

// value moves into function and pointers outside are no longer valid
fn takes_ownership(some_string: String) {
  println!("takes_ownership({})", some_string);
}

// i32 supports copy so new value is created on the stack
fn makes_copy(some_int: i32) {
  println!("makes_copy({})", some_int);
}

fn gives_ownership() -> String {
  let some_string = String::from("Hello");
  some_string
}
fn takes_and_gives_back(a_string: String) -> String {
  a_string
}

// you can return multiple values using a tuple
fn calculate_len(s: String) -> (String, usize) {
  let len = s.len();
  (s, len)
}