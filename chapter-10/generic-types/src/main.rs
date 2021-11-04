
#[derive(Debug)]
struct Point<T> {
  x: T,
  y: T,
}

impl<T> Point<T> {
  fn x(&self) -> &T {
    &self.x
  }
}
impl Point<f32> {
  fn distance_from_origin(&self) -> f32 {
    (self.x.powi(2) + self.y.powi(2)).sqrt()
  }
}

#[derive(Debug)]
struct PointMixed<T, U> {
  x: T,
  y: U,
}
impl<T, U> PointMixed<T, U> {
  fn mixup<V, W>(self, other: PointMixed<V, W>) -> PointMixed<T, W> {
    PointMixed { x: self.x, y: other.y }
  }
}

fn main() {
  let number_list = vec![34, 50, 25, 100, 65];

  // let result = largest_i32(&number_list);
  let result = largest(&number_list);
  println!("The largest number is {}", result);

  let char_list = vec!['y', 'm', 'a', 'q'];

  // let result = largest_char(&char_list);
  let result = largest(&char_list);
  println!("The largest char is {}", result);

  let int_point = Point { x: 2, y:0 };
  let float_point = Point { x: 1.0, y: 3.0 };

  // We can't mix types with a single generic type but we can do if more than
  // one genric has been defined
  // let mix_point = Point { x:0, y: 4.0};
  let mix_point = PointMixed { x: 0, y: 4.0 };

  println!("Int Point {:?}, Float Point: {:?}, Mix Point: {:?}", int_point, float_point, mix_point);

  println!("Direct X: {}, Getter X: {}", int_point.x, int_point.x());

  println!("Distance: {}", float_point.distance_from_origin());

  let mix_point2 = PointMixed { x: "Hello", y: "World" };
  let mix_point3 = mix_point.mixup(mix_point2);

  println!("Mix Point3 {:?}", mix_point3);

}

// The following code won't work as we can't make direct comparisons with
// generic types, to do this we'll need to use traits.
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
  let mut largest = list[0];

  for &item in list {
    if item > largest {
      largest = item;
    }
  }

  largest
}
// -- or --
// This isn't create as it could potentially making more heap allocations
// when the data types are heap owned
// fn largest<T: PartialOrd + Clone>(list: &[T]) -> &T {
//   let mut largest = &list[0];

//   for item in list {
//     if item > largest {
//       largest = item;
//     }
//   }

//   largest
// }
// -- or --
// NOTE: Runs into life times errors which haven't been covered yet
// fn largest<T: PartialOrd>(list: &[&T]) -> &T {
//   let mut largest = &list[0];

//   for item in list {
//     if item > largest {
//       largest = item;
//     }
//   }

//   largest
// }

// fn largest_i32(list: &[i32]) -> i32 {
//   let mut largest = list[0];

//   for &item in list {
//     if item > largest {
//       largest = item;
//     }
//   }

//   largest
// }

// fn largest_char(list: &[char]) -> char {
//   let mut largest = list[0];

//   for &item in list {
//     if item > largest {
//     largest = item;
//     }
//   }

//   largest
// }
