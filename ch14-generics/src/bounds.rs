use std::fmt::{Debug, Display};
struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

fn red<T: Red>(_: &T)  -> &'static str { "red" }
fn blue<T: Blue>(_: &T)  -> &'static str { "blue" }

fn compare_prints<T: Debug + Display>(t: &T) {
  println!("Debug: `{:?}`", t);
  println!("Display: `{}`", t);
}

fn compare_types<T: Debug, U:Debug>(t: &T, u: &U) {
  println!("t: `{:?}`", t);
  println!("u: `{:?}`", u);
}

// Where clauses can clean bounds
// impl <A: TraitB + TraitC, D: TraitE + TraitF> MyTrait<A, D> for YourType {}

// impl <A, D> MyTrait<A, D> for YourType where
//   A: TraitB + TraitC,
//   D: TraitE + TraitF
//   {}

trait PrintInOption {
  fn print_in_option(self);
}

// Because we would otherwise have to express this as `T: Debug` or 
// use another method of indirect approach, this requires a `where` clause:
impl<T> PrintInOption for T where
    Option<T>: Debug {
    // We want `Option<T>: Debug` as our bound because that is what's
    // being printed. Doing otherwise would be using the wrong bound.
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

fn main() {
  let cardinal = Cardinal;
  let blue_jay = BlueJay;
  let _turkey = Turkey;

  println!("A cardinal is {}", red(&cardinal));
  println!("A blue jay is {}", blue(&blue_jay));
  // println!("A trukey is {}", red(&_turkey));

  let string = "words";
  let array = [1, 2, 3];
  let vec = vec![1, 2, 3];

  compare_prints(&string);
  compare_types(&array, &vec);

  vec.print_in_option();
}