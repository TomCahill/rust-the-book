use minigrep::*;

#[test]
fn case_sensitive() {
  let query = "duct";
  let contents = "Rust:\nsafe, fast, productive.\nPick Three.";

  assert_eq!(vec!["safe, fast, productive."], search(query, contents));
}

#[test]
fn case_insensitive() {
  let query = "RuSt";
  let contents = "Rust:\nsafe, fast, productive.\nPick Three.\nTrust me.";

  assert_eq!(vec!["Rust:", "Trust me."], search_case_insenstive(query, contents));
}