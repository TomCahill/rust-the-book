// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }

pub trait Summary {
  fn summarize_author(&self) -> String;

  fn summarize(&self) -> String {
    format!("Read more from {}...", self.summarize_author())
  }
}

pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
      format!("{}", self.author)
    }
}

pub struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool,
}

impl Summary for Tweet {
  fn summarize_author(&self) -> String {
    format!("@{}", self.username)
  }
  fn summarize(&self) -> String {
    format!("{}: {}", self.username, self.content)
  }
}

pub fn notify(item: &impl Summary) {
  println!("{}", item.summarize())
}

pub fn notify_no_sugar<T: Summary>(item: &T) {
  println!("Breaking News! {}", item.summarize())
}