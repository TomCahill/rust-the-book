#![allow(unused)]
enum IPAddrKind {
  V4,
  V6
}

// struct IPAddr {
//   kind: IPAddrKind,
//   address: String,
// }
// - vs -
enum IPAddr {
  V4(String),
  V6(String),
}

// NOTE: IpAddr is an existing definition within the standard libary

enum IPAddrNumeric {
  V4(u8, u8, u8, u8),
  V6(String),
}

#[derive(Debug)]
enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColour(i32, i32, i32)
}
impl Message {
  fn call(&self) {
    println!("{:?}", &self);
  }
}

fn main() {
  let four = IPAddrKind::V4;
  let six = IPAddrKind::V6;

  // let home = IPAddr { kind: IPAddrKind::V4, address: String::from("127.0.0.1") };
  // let loopback = IPAddr { kind: IPAddrKind::V6, address: String::from("::1") };
  // - vs -
  let home = IPAddr::V4(String::from("127.0.0.1"));
  let loopback = IPAddr::V6(String::from("::1"));

  let home = IPAddrNumeric::V4(127, 0, 0, 1);

  let m = Message::Write(String::from("hello"));
  let m = Message::Move { x:32, y:12};
  m.call();

  // No null types but the standard libary defines a enum which can be used to impliment
  // the same concept
  let some_number = Some(5);
  let some_string = Some("a string");

  let absent_number: Option<i32> = None;

  let x = 5;
  let y: Option<i32> = Some(51);

  // This will result in a error because y has the type Option which contains a i32, rust
  // instead provides some helpful functionally for handling the access of a potentially
  // null value 
  // let sum = x + y;
  let sum = x + y.unwrap_or(0);


  println!("End of program");
}

