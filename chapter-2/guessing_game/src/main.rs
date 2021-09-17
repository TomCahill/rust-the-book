use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
  println!("Guess the number!");

  let secret_num = rand::thread_rng().gen_range(1..11);

  loop {
    println!("Please input your guess:");

    let mut guess = String::new();

    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line");

    let guess = guess.trim();

    if guess.is_empty() {
      println!("Please enter a input!");
        continue
    }

    let guess: u32 = match guess.parse() {
      Ok(num) => num,
      Err(_) => {
        println!("Please enter a number!");
        continue
      }
    };

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_num) {
      Ordering::Less => println!("Too Small!"),
      Ordering::Greater => println!("Too Big!"),
      Ordering::Equal => {
        println!("Winner");
        break
      },
    }
  }
}
