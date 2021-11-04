use std::fmt::Display;
use traits::{Summary, Tweet, NewsArticle, notify, notify_no_sugar};

struct Pair<T> {
  x: T,
  y: T,
}
impl<T> Pair<T> {
  fn new(x: T, y: T) -> Self {
    Self {x, y}
  }
}
impl<T: Display + PartialOrd> Pair<T> {
  fn cmp_display(&self) {
    if self.x >= self.y {
      println!("The largest member is x = {}", self.x);
    } else {
      println!("The largest member is y = {}", self.y);
    }
  }
}


fn main() {
  let tweet = Tweet {
    username: String::from("the_computer"),
    content: String::from("Beep boop boop"),
    reply: false,
    retweet: false,
  };

  println!("1 new tweet: {}", tweet.summarize());

  let article = NewsArticle {
    headline: String::from("Penguins win the Stanley Cup Championship!"),
    location: String::from("Pittsburgh, PA, USA"),
    author: String::from("Iceburgh"),
    content: String::from(
        "The Pittsburgh Penguins once again are the best \
          hockey team in the NHL.",
    ),
  };

  println!("New article available! {}", article.summarize());

  notify(&article);
  notify_no_sugar(&article);

  let a = Pair { x: 1, y: 2 };
  let b = Pair { x: false, y: true };

  let c = Pair::new(23, 12);

  a.cmp_display();
  b.cmp_display();
  c.cmp_display();

  println!("toString: {}", 3.to_string());

}