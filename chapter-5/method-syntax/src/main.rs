#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

// Using impl we can add methods to structs
impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }

  fn sq(size: u32) -> Rectangle {
    Rectangle {
      width: size,
      height: size
    }
  }
}

// You can even have mutiple impl for a struct
impl Rectangle {
  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
}

fn main() {
  let rect1 = Rectangle { width: 30, height: 50 };
  let rect2 = Rectangle { width: 10, height: 40 };
  let rect3 = Rectangle { width: 60, height: 45 };

  println!(
    "The area of the rectangle is {} square pixels.",
    rect1.area()
  );

  println!(
    "Can rect1 hold rect 2? {}",
    rect1.can_hold(&rect2),
  );
  println!(
    "Can rect1 hold rect 3? {}",
    rect1.can_hold(&rect3),
  );

  let square = Rectangle::sq(32);
  println!(
    "The area of the square is {} square pixels, dimensions are {:?}.",
    square.area(),
    square,
  );
}
