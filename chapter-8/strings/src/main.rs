fn main() {
  let mut _s = String::new();

  let data = "init content";

  let _s = data.to_string();

  let _s = "init contents".to_string();

  let _hello = String::from("السلام عليكم");
  let _hello = String::from("Dobrý den");
  let _hello = String::from("Hello");
  let _hello = String::from("שָׁלוֹם");
  let _hello = String::from("नमस्ते");
  let _hello = String::from("こんにちは");
  let _hello = String::from("안녕하세요");
  let _hello = String::from("你好");
  let _hello = String::from("Olá");
  let _hello = String::from("Здравствуйте");
  let _hello = String::from("Hola");

  let mut s = String::from("foo");
  s.push_str("bar");
  assert_eq!("foobar", s);

  let mut s1 = String::from("apples");
  let s2 = "pears";
  s1.push_str(s2);
  assert_eq!("applespears", s1);
  assert_eq!("pears", s2);

  let mut s = String::from("LO");
  // NOTE: This takes a char
  s.push('L');
  assert_eq!("LOL", s);

  let s1 = String::from("Hello, ");
  let s2 = String::from("world!");
  let s3 = s1 + &s2;
  // S1 gets moved here so we're unable to use it from this point forward
  // S2 is fine as it was referenced
  // assert_eq!("Hello, ", s1);
  assert_eq!("world!", s2);
  assert_eq!("Hello, world!", s3);

  let tic = String::from("tic");
  let tac = String::from("tac");
  let toe = String::from("toe");
  let s = tic + "-" + &tac + "-" + &toe;
  assert_eq!("tic-tac-toe", s);

  // -vs-

  let tic = String::from("tic");
  let tac = String::from("tac");
  let toe = String::from("toe");

  let s = format!("{}-{}-{}", tic, tac, toe);
  // NOTE: Using format doesn't take ownership of any of the params
  assert_eq!("tic", tic);
  assert_eq!("tac", tac);
  assert_eq!("toe", toe);
  assert_eq!("tic-tac-toe", s);

  // Unicode scalar values are stored in two bytes there for if we're
  // trying to read out the bytes that make them up we need to make sure
  // we stick within the bounty for each char
  let hello = "Здравствуйте";
  let s = &hello[0..4];
  assert_eq!("Зд", s);
  // This will fail if uncommented 
  // let s = &hello[0..3];

  // Due to the char bountry we're unable to directly reference a index
  // within a string as you'd expect (&str[0]) it's also not clear what
  // you'd be expecting to be returned when trying to access a char this
  // Instead rust offers more speific ways of accesing what you'd want

  println!("Each Char in string: ");
  for c in "this is a string".chars() {
    println!("{}", c);
  }
  println!("");

  println!("Each byte in string: ");
  for b in "this is a string".bytes() {
    println!("{}", b);
  }
  println!("");
}
