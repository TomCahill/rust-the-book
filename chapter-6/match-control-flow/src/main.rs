#[derive(Debug)]
enum Country {
  England,
  Scotland,
  Wales,
}

enum Coin {
  Penny,
  Two,
  Five,
  Ten,
  Twenty,
  Fifty(Country),
  Pound,
}

fn main() {
  let pence100 = value_in_pence(Coin::Pound);
  println!("Hello, world! {}", pence100);

  println!("Penny {}", value_in_pence(Coin::Penny));
  println!("Scottish fifty {}", value_in_pence(Coin::Fifty(Country::Scotland)));
  println!("English fifty {}", value_in_pence(Coin::Fifty(Country::England)));

  let five = Some(5);
  let six = plus_one(five);
  let none = plus_one(None);
  println!("five={:?}, six={:?}, none={:?}", five, six, none);

  let count_von_count = 0u8;

  // In this example we're going to use placeholder match so we don't have to express
  // every possible combination of u8
  match count_von_count {
    1 => println!("One"),
    3 => println!("Three"),
    5 => println!("Five"),
    7 => println!("Seven"),
    _ => println!("I don't know that number")
  }

}

fn value_in_pence(coin: Coin) -> u8 {
  match coin {
    Coin::Penny => {
      println!("Lucky Penny");
      1
    },
    Coin::Two => 2,
    Coin::Five => 5,
    Coin::Ten => 10,
    Coin::Twenty => 20,
    Coin::Fifty(country) => {
      println!("Country fifty from {:?}!", country);
      50
    },
    Coin::Pound => 100,
  }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    None => None,
    Some(i) => Some(i + 1),
  }
}