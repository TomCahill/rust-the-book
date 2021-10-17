use std::io;
use std::cmp::Ordering;

use rand::Rng;

fn main() {
  guess_game();
}

fn guess_game() {
  println!("I've just picked a number between 1 and 100");

  let secret_number = rand::thread_rng().gen_range(1..101);

  loop {
    println!("Take a guess");

    let mut guess = String::new();

    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to get input");

    let guess: i32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };

    if guess < 1 || guess > 100 {
      println!("The number is between 1 and 100, take another guess.");
      continue;
    }

    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Ha, Wrong.. Too small!"),
      Ordering::Greater => println!("Ha Wrong.. Too big!"),
      Ordering::Equal => {
        println!("Ok.. you win!! I bet you've cheated somehow");
        break;
      }
    }

  }
}