use std::vec;

#[derive(Debug)]
enum Cell {
  Int(i32),
  Float(f64),
  Text(String),
}

fn main() {
  // Define types
  let v: Vec<i32> = Vec::new();

  // Infer type from values via shortcut
  let v = vec![1, 2, 3];

  // Infer types from values after declaration
  let mut v = Vec::new();
  v.push(5);

  let v = vec![1,2,3,4,5];

  let third: &i32 = &v[2];
  println!("The third element is {}", third);

  match v.get(2) {
    Some(third) => println!("3rd value {}", third),
    None => println!("No 3rd index in vector"),
  }

  let v = vec![1,2,3,4,5];

  // Trying to borrow a value that doesn't exist will result in a panic
  // let this_will_panic = &v[100];

  // Using the get method will return a result value which can be handled
  let does_not_panic = v.get(100);

  let v = vec![100, 32, 57];
  for i in &v {
    println!("{}", i);
  }

  let mut v = vec![23, 19, 82];
  println!("Before Mut: {:?}", v);
  for i in &mut v {
    *i += 50;
  }
  println!("After Mut: {:?}", v);

  // We can store a mix of types in an array using enums
  let row = vec![
    Cell::Int(52),
    Cell::Text(String::from("green")),
    Cell::Float(3.14159),
  ];
  println!("Mix Vec: {:?}", row);
}
