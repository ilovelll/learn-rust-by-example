use std::fmt;
fn main() {
  println!("{} days", 31);
  println!("{} days", 31u8); // with suffix
  println!("${} ", 1_100_100); //with a separator

  println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

  // named arguments
  println!("{subject} {verb} {object}",
    object="the lazy dog",
    subject="the quick fox",
    verb="jumps over");

  // Special format after a `:`
  println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

  // will output "0042"
  println!("{:04}", 42);
  // will output "     1"
  println!("{number:>width$}", number=1, width=6);

  // will output "000001"
  println!("{number:>0width$}", number=1, width=6);

  // will output "     1"
  println!("{number:width$}", number=1, width=6);

  // will output "x     "
  println!("{number:width$}", number="x", width=6);

  // will output "x     "
  println!("{number:<width$}", number="x", width=6);

  // will output "     x"
  println!("{number:>width$}", number="x", width=6);

  // will output "Hello x is 0.01000"
  println!("Hello {} is {number:.prec$}", "x", prec = 5, number = 0.01);

  #[allow(dead_code)]
  #[derive(Debug)]
  struct Structure(i32);
  
  #[derive(Debug)]
  struct Deep(Structure);

  impl fmt::Display for Structure{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{}", self.0)
    }
  }
  println!("This struct `{}` won't print...", Structure(3));

  let pi = 3.141592;
  println!("Pi is roughly {:.3}", pi);
  println!("{1:?} {0:?} is the {actor:?} name.",
            "Slater",
            "Christian",
            actor="actor's");

  println!("Now {:?} will print!", Structure(3));
  println!("Now {:?} will print!", Deep(Structure(3)));

  #[derive(Debug)]
  struct Person<'a> {
      name: &'a str,
      age: u8
  }

  let name = "Peter";
  let age = 24;
  let peter = Person{ name, age };

  // pretty printing with {:#?} 
  println!("{:#?}", peter);
}