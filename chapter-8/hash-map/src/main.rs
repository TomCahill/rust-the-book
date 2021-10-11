use std::{collections::HashMap, vec};

fn main() {
  // Like vectors, hash maps need to have typed keys and values
  let mut scores = HashMap::new();
  scores.insert(String::from("Red"), 10);
  scores.insert(String::from("Blue"), 25);

  let teams = vec![String::from("Blue"), String::from("red")];
  let init_scores = vec![35, 21];

  let mut scores: HashMap<_, _> = teams.into_iter().zip(init_scores.into_iter()).collect();

  let blue_score = scores.get("Blue");
  println!("Blue score: {}", blue_score.unwrap_or(&0));
  println!("");

  println!("All Scores:");
  println!("-------------");
  for (key, value) in &scores {
    println!("{}: {}", key, value);
  }

  scores.insert(String::from("Blue"), 12);

  println!("{:?}", scores);

  scores.entry(String::from("Yellow")).or_insert(50);
  scores.entry(String::from("Blue")).or_insert(50);

  println!("{:?}", scores);

  let text = "She sells seashells by the seashore,
    The shells she sells are seashells, I’m sure.
    So if she sells seashells on the seashore,
    Then I’m sure she sells seashore shells.";

  let mut map = HashMap::new();

  for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
  }

  println!("{:?}", map);

}
