struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool,
}

// Struct should own all data so we're unable to pass through references
// uncommenting the line below will result in errors
// struct UserRef {
//   username: &str,
//   email: &str,
//   sign_in_count: u64,
//   active: bool,
// }

struct Colour(i16, i16, i16);
struct Point(i32, i32, i32);

fn main() {
  let user1 = User {
    username: String::from("username123"),
    email: String::from("someperson123@example.com"),
    active: true,
    sign_in_count: 1
  };

  let mut user1 = User {
    username: String::from("username123"),
    email: String::from("someperson123@example.com"),
    active: true,
    sign_in_count: 1
  };
  user1.email = String::from("anotheremail@example.com");

  let user1 = build_user(String::from("jeff@example.com"), String::from("jeff123"));

  // Spread
  let user2 = User {
    username: String::from("username123"),
    email: String::from("someperson123@example.com"),
    ..user1
  };


  let black = Colour(0, 0, 0);
  let origin = Point(127, 0, 0);
}

fn build_user(email: String, username: String) -> User {
  User {
    email,
    username,
    active: false,
    sign_in_count: 0,
  }
}
