extern crate rary;

fn main() {
  rary::public_function();

  // Error! `private_function` is private
  //rary::private_function();

  rary::indirect_access();
}

//  rustc executable.rs --extern rary=library.rlib && ./executable