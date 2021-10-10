
// Idiomatic approach to referencing things within libs note
// we can use this approach if it will result in a naming clash
use std::collections::HashMap;
// - vs -
use std::fmt;
use std::io;
// - vs -
use std::{io, fmt};

use std::io::Result as IoResult;


// use std::io;
// use std::io::Write;
// -vs-
// use std::io::{self, Write};

// Glob
use std::collections::*;

// Re-export 

mod front_of_house {
  pub mod hosting {
    pub fn add_to_waitlist() {}
  }
}

// Absolute
// use crate::front_of_house::hosting;

// Relative
use self::front_of_house::hosting;

// Idiomatic
use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
  hosting::add_to_waitlist();

  add_to_waitlist();

  let mut map = HashMap::new();
  map.insert(1, 2);
}

// Result would name clash if we didn't reference parent module
// fn func1() -> fmt::Result {}
// fn func2() -> io::Result<()> {}

// fn func3() -> IoResult<()> {}