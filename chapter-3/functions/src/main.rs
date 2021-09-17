fn main() {
  println!("Hello, world!");

  another_function(23);

  expression_example(2);

  println!("five() returns: {}", five());
  println!("plus_one(5) returns: {}", plus_one(5));
}

fn another_function(x: i32) {
  println!("Another function with i32 param: {}.", x);
}

fn expression_example(inc: i32) {
  let x = 5;

  let y = {
      let x = 3;
      x + inc
  };

  println!("expression_example y is: {}, x remains untouched because of block scoping: {}", y, x);
}

fn five() -> i32 { 5 }
fn plus_one(x: i32) -> i32 { x + 1 }