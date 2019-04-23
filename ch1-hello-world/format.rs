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


  // struct Complex(f32, f32);
  #[derive(Debug)]
  struct Complex {
    real: f32,
    imag: f32
  }

  impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{0} +{1}i", self.real, self.imag)
    }
  }
  let c = Complex{ real: 3.3, imag: 7.2};
  println!("Display: {}", c);
  println!("Debug: {:?}", c);


  struct List(Vec<i32>);

  impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      let vec = &self.0;

      // use `?` to check no error and continue;
      write!(f, "[")?;

      for (count, v) in vec.iter().enumerate() {
        if count != 0 {
          write!(f, ", ")?;
        }
        write!(f, "{}: {}", count, v)?;
      }

      write!(f, "]")
    }
  }


  let v = List(vec![1,2,3]);
  println!("{}", v);


  #[derive(Debug)]
  struct Color {
    red: u8,
    green: u8,
    blue: u8,
  }

  impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "RGB ({0}, {1}, {2}) 0x{0:02X}{1:02X}{2:02X}", self.red, self.green, self.blue)
    }
  }

  for color in [
    Color { red: 128, green: 255, blue: 90 },
    Color { red: 0, green: 3, blue: 254 },
    Color { red: 0, green: 0, blue: 0 },
  ].iter() {
      // Switch this to use {} once you've added an implementation
      // for fmt::Display
    println!("{:?}", *color);
    println!("{}", *color);
  }
}