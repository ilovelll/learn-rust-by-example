use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::io::prelude::*;
use std::path::Path;
use std::fs::OpenOptions;

static LOREM_IPSUM: &str =
    "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
";

fn main() -> std::io::Result<()> {
  let path = Path::new("Cargo.toml");
  let display = path.display();

  let mut file = match File::open(&path) {
    Err(why) => panic!("couldn't open {}: {}", display, why.description()),
    Ok(file) => file,
  };

  let mut s = String::new();
  match file.read_to_string(&mut s) {
    Err(why) => panic!("couldn't read {}: {}", display, why.description()),
    Ok(_) => print!("{} contains:\n{}", display, s),
  }

  let mut file = OpenOptions::new()
              .read(true)
              .write(true)
              .create(true)
              .open("lorem_ipsum2.txt")?;
  

  file.write_all(LOREM_IPSUM.as_bytes())?;

  let path = Path::new("lorem_ipsum.txt");
  let display = path.display();

  let mut file = match File::create(&path) {
    Err(why) => panic!("couldn't create {}: {}", display, why.description()),
    Ok(file) => file,
  };

  match file.write_all(LOREM_IPSUM.as_bytes()) {
    Err(why) => panic!("cound't write to {}: {}", display, why.description()),
    Ok(_) => println!("successfully wrote to {}", display),
  }

  let file = File::open("./lorem_ipsum.txt")?;
  let lines = io::BufReader::new(file).lines();
  for line in lines {
    if let Ok(s) = line {
      println!("{}", s);
    }
  }
  Ok(())
}