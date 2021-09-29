fn main() {
  let mut s = String::from("Hello world");
  println!("The string is '{}'", s);

  let word = first_word(&s);

  println!("First word of '{}' is {}", s, word);

  s.clear();

  let s = String::from("hello");

  let slice_start = &s[..2];
  let slice_end = &s[2..];
  println!("Slice of '{}' {}..{}", s, slice_start, slice_end);

  let s = String::from("banana slice");

  let word = first_word_slice(&s);

  println!("the first word is {}", word);
  
  let string_ref = String::from("hello moto");

  // Because we have used &str as for first_word_slice arg this
  // allows us to take advantage of using both strings and slices
  // this allows the function to be more flexable in the cases it 
  // can be used
  let word = first_word_slice(&string_ref[0..6]);
  println!("Slice of word {}", word);
  let word = first_word_slice(&string_ref[..]);
  println!("Slice of word {}", word);
  let word = first_word_slice(&string_ref);
  println!("Slice of word {}", word);

  let string_lit = "Floop Woop Doop";
  let word = first_word_slice(&string_lit[0..6]);
  println!("Slice of word {}", word);
  let word = first_word_slice(&string_lit[..]);
  println!("Slice of word {}", word);
  let word = first_word_slice(&string_lit);
  println!("Slice of word {}", word);

  let arr = [1,2,3,4,5,6];
  println!("arr {:?}", arr);

  let slice = &arr[3..5];
  println!("Slice of arr {:?}", slice);

  assert_eq!(slice, &[4, 5]);
}

fn first_word(s: &String) -> usize {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return i
    }
  }

  s.len()
}
fn first_word_slice(s: &str) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[..i];
    }
  }

  &s[..]
}