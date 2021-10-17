use std::error::Error;

use std::fs;
use std::fs::File;

use std::io;
use std::io::Read;
use std::io::ErrorKind;

fn main() -> Result<(), Box<dyn Error>> {
  let filename = "hello.txt";

  let f = File::open(filename);

  let _f = match f {
    Ok(file) => file,
    Err(error) => match error.kind() {
      ErrorKind::NotFound => match File::create(filename) {
        Ok(fc) => fc,
        Err(e) => panic!("Unable to create file '{}' {:?}", filename, e)
      },
      other_error => {
        panic!("Unable to open file: {:?}", other_error)
      }
    }
  };

  let _f = File::open(filename).unwrap_or_else(|error| {
    if error.kind() == ErrorKind::NotFound {
      File::create(filename).unwrap_or_else(|error| {
        panic!("Unable to create file '{}' {:?}", filename, error)
      })
    } else {
      panic!("Unable to open file: {:?}", error)
    }
  });

  let _f = File::open(filename).unwrap();

  let _f = File::open(filename).expect("Unable to open filename");

  match read_username_from_file() {
    Ok(s) => println!("read_username_from_file: {:?}", s),
    Err(e) => println!("error read_username_from_file: {:?}", e),
}

  match read_name_from_file() {
    Ok(s) => println!("read_name_from_file: {:?}", s),
    Err(e) => println!("error read_name_from_file: {:?}", e),
  }

  match read_email_from_file() {
    Ok(s) => println!("read_email_from_file: {:?}", s),
    Err(e) => println!("error read_email_from_file: {:?}", e),
  }

  match read_mobile_from_file() {
    Ok(s) => println!("read_mobile_from_file: {:?}", s),
    Err(e) => println!("error read_mobile_from_file: {:?}", e),
  }

  // This can only be used in functions that return a Result
  // We've changed main to reutrn a result using a trait object to handle
  // returning any "kind of error", because of this main now returns
  let f = File::open("hello.txt")?;

  Ok(())
}

fn read_username_from_file() -> Result<String, io::Error> {
  let f = File::open("username.txt");

  let mut f = match f {
    Ok(file) => file,
    Err(e) => return Err(e),
  };

  let mut s = String::new();

  match f.read_to_string(&mut s) {
    Ok(_) => Ok(s),
    Err(e) => Err(e),
  }
}
fn read_name_from_file() -> Result<String, io::Error> {
  let mut f = File::open("name.txt")?;
  let mut s = String::new();
  f.read_to_string(&mut s)?;
  Ok(s)
}
fn read_email_from_file() -> Result<String, io::Error> {
  let mut s = String::new();

  File::open("email.txt")?.read_to_string(&mut s)?;

  Ok(s)
}
fn read_mobile_from_file() -> Result<String, io::Error> {
  fs::read_to_string("mobile.txt")
}