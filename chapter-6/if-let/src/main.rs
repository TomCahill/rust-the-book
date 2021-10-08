#[derive(Debug)]
enum Country {
  England,
  Scotland,
}
#[derive(Debug)]
enum Coin {
  Penny,
  Pound(Country)
}

fn main() {
  let some_u8_val = Some(3);

  match some_u8_val {
    Some(3) => println!("Three"),
    _ => (),
  }
  // Rust provides a if let we can be used as a alternative to match when directly
  // trying to match on a condition
  if let Some(3) = some_u8_val {
    println!("Three")
  }

  let coin = Coin::Penny;

  // NOTE in these examples we need to borrow coin otherwise it would be moved
  // and injested by the expressions

  let mut count = 0;
  match &coin {
    Coin::Pound(country) => println!("State quarter from {:?}!", country),
    _ => count += 1,
  }
  println!("Count is now {:?}!", count);
  // - vs -
  if let Coin::Pound(country) = &coin {
    println!("country pound coin from {:?}!", country);
  } else {
    count += 1;
  }
  println!("Count is now {:?}!", count);
}
