use std::fmt::Display;

struct ImportantExcerpt<'a> {
	part: &'a str,
}

fn main() {
	println!("Hello, world!");

	let string1 = String::from("abcd");
	let string2;
	let result;

	{
		string2 = String::from("xyz");
		result = longest(string1.as_str(), string2.as_str());
	}

	println!("Longest: {}", result);

	let novel = String::from("Call me Ishmael. Some years ago...");
	let first_sentence = novel.split('.').next().expect("Could not find a '.'");
	let i = ImportantExcerpt {
		part: first_sentence,
	};

	println!("ImportantExcerpt: {}", i.part);

	println!("First Word: {}", first_word(&novel));

	let result = longest_with_an_announcement("abc","a", "Hello World");
	println!("longest_with_an_announcement: {}", result);
}

// fn longest(x: &str, y: &str) -> &str { - After Rule 1
// fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str { - After Rule 2
// This will result in a error because Rule 3 can't be applied due to mutiple params

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
	if x.len() > y.len() {
		x
	} else {
		y
	}
}

fn first_word(s: &str) -> &str {
// fn first_word<'a>(s: &'a str) -> &str {    - After Rule 1
// fn first_word<'a>(s: &'a str) -> &'a str { - After Rule 2
	let bytes = s.as_bytes();

	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			return &s[0..i];
		}
	}

	&s[..]
}

fn longest_with_an_announcement<'a, T> (
	x: &'a str,
	y: &'a str,
	ann: T,
) -> &'a str
where
	T: Display,
{
	println!("Announcement! {}", ann);
	if x.len() > y.len() {
		x
	} else {
		y
	}
}