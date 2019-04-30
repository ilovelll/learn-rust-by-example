mod inaccessible;

pub mod nested;

pub fn function() {
  println!("called `dir_mod::function()`");
}

fn private_function() {
  println!("called `dir_mod::private_function()`");
}

pub fn indirect_access() {
  println!("called `dir_mod::indirect_access()`, that ");
  private_function();
}