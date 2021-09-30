#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

fn main() {
  let width1 = 30;
  let height1 = 50;

  println!(
    "The area of the rectangle is {} square pixels.",
    area(width1, height1)
  );

  // let rect1 = (30, 50);

  println!(
    "The area of the rectangle is {} square pixels.",
    area_by_tuple((30, 50))
  );

  let rect1 = Rectangle {
    width: 40,
    height: 40,
  };

  println!(
    "The area of the rectangle is {} square pixels.",
    area_by_struct(&rect1)
  );

  // This only works as the struct has #[derive(Debug)] defined
  println!(
    "Rectangle {:?}.",
    rect1
  );

  // Passing the has between gives a little more formatted debug output
  println!(
    "Rectangle {:#?}.",
    rect1
  );
}

fn area(width: u32, height: u32) -> u32 {
  width * height
}
fn area_by_tuple(dimensions: (u32, u32)) -> u32 {
  dimensions.0 * dimensions.1
}
fn area_by_struct(rectangle: &Rectangle) -> u32 {
  rectangle.width * rectangle.height
}