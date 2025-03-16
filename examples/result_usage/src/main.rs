use std::fs::File;
use std::io::ErrorKind;
fn main() {
  let result = File::open("hello.txt");
  let ret = match result {
    Ok(file) => file,
    Err(error) => match error.kind() {
      ErrorKind::NotFound => match File::create("hello.txt") {
        OK(fc) => fc,
        Err(e) => panic!("Problem creating the file: {e:?}"),
      },
      other_error => {
        panic!("Problem opening the file: {other_error:?}");
      }
    },
  };
  
  let greeting_file = File::open("hello.txt")?; // incorrect!
}

fn read_username_from_file1() -> Result<String, io::Error> {
  let mut username = String::new();
  File::open("hello.txt")?.read_to_string(&mut username)?;
  Ok(username)
}

fn read_username_from_file2() -> Result<String, io::Error> {
  fs::read_to_string("hello.txt")
}