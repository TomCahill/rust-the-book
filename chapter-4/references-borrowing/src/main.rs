fn main() {
  let s = String::from("Hello");
  let len = calc_len(&s);

  println!("The length of '{}' is {}.", s, len);

  // Uncommting this will result in an error as borrowed values are immutable
  // addWorld(&s);

  let mut s = String::from("Hello");
  add_world_mut(&mut s);

  let len = calc_len(&s);

  println!("String is now '{}' and length is {}.", s, len);

  // Uncommeting this will result in an error, we're unable to more than one
  // mutable reference in a scope
  // let r1 = &mut s;
  // let r2 = &mut s;
  // println!("{}, {}", r1, r2);

  // Here we are able to have more than one mutable reference because we've
  // created a new block scope
  {
    let r1 = &mut s;
    println!("Inside block scope {}.", r1);
  }

  let r2 = &mut s;
  println!("in main scope {}.", r2);

  // NOTE: This the follow won't work because the reference made before the
  // block scope is included within the block scope
  // let r1 = &mut s;
  // {
  //   let r2 = &mut s;
  //   println!("within block scope {} {}", r1, r2);
  // }

  // We're also note able to mix mutable and immutable references within the
  // same scope
  // let r1 = &s;
  // let r2 = &s;
  // let r3 = &mut s;
  // println!("{}, {}, and {}", r1, r2, r3);

  let r1 = &s;
  let r2 = &s;
  println!("{} and {}", r1, r2);

  // here the mixing is fine because the immutable references dont overlap
  let r3 = &mut s;
  println!("{}", r3);

  // Unccoment the following line to get an erro due to a dangling reference
  // let _dangling_ref = dangle();

  let _no_dangle = no_dangle();
}

fn calc_len(s: &String) -> usize {
  s.len()
}

// fn addWorld(s: &String) {
//   s.push_str(", world");
// }

fn add_world_mut(s: &mut String) {
  s.push_str(", world");
}

// fn dangle() -> &String {
//   let s = String::from("Hello");

//   &s
// }
fn no_dangle() -> String {
  let s = String::from("Hello");

  s
}