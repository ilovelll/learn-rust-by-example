fn main() {
  fn function (i: i32) -> i32 { i + 1}

  let closure_annotated = |i: i32| -> i32 {i + 1};
  let closure_inferred = |i| i + 1;

  println!("function: {}", function(1));
  println!("closure_anotated: {}", closure_annotated(1));
  println!("closure_inferred: {}", closure_inferred(1));

  let one = || 1;
  println!("closure returing one: {}", one());

  let color = "green";
  let print = || println!("`color`:{}", color);
  print();
  print();

  let mut count = 0;

  let mut inc = || {
    count += 1;
    println!("`count`: {}", count);
  };
  inc();
  inc();
  let _reborrow = &mut count;
  println!("{}", _reborrow);

  let movable = Box::new(3);

  let consume = || {
    println!("`moveable`: {:?}", movable);
    std::mem::drop(movable);
  };

  consume();
  // movable has been move
  // consume();

  let haystack = vec![1,2,3];
  let contains = move |needle| haystack.contains(needle);
  println!("{}", contains(&1));
  println!("{}", contains(&4));
  // use after `move` will cause error
  // println!("{}", haystack.len());

  // As input params
  let greeting = "hello";
  let mut farewell = "goodbye".to_owned();
  apply(|| {
    println!("I said {}", greeting);
    farewell.push_str("!!!");
    println!("Then I screamed {}.", farewell);
    println!("Now I can sleep. zzzz");
    std::mem::drop(farewell);
    });

  let x = apply_to3(|x|{
    x * 2
  });
  println!("3 doubled: {}", x);

  let fn_plain = create_fn();
  let mut fn_mut = create_fnmut();

  fn_plain();
  fn_mut();

//pub trait Iterator {
//     // The type being iterated over.
//     type Item;

//     // `any` takes `&mut self` meaning the caller may be borrowed
//     // and modified, but not consumed.
//     fn any<F>(&mut self, f: F) -> bool where
//         // `FnMut` meaning any captured variable may at most be
//         // modified, not consumed. `Self::Item` states it takes
//         // arguments to the closure by value.
//         F: FnMut(Self::Item) -> bool {}
// }

  let vec1 = vec![1, 2, 3];
  let vec2 = vec![4, 5, 6];

  // `iter()` for vecs yields `&i32`. Destructure to `i32`.
  println!("2 in vec1: {}", vec1.iter()     .any(|&x| x == 2));
  // `into_iter()` for vecs yields `i32`. No destructuring required.
  println!("2 in vec2: {}", vec2.into_iter().any(| x| x == 2));

  let array1 = [1, 2, 3];
  let array2 = [4, 5, 6];

  // `iter()` for arrays yields `&i32`.
  println!("2 in array1: {}", array1.iter()     .any(|&x| x == 2));
  // `into_iter()` for arrays unusually yields `&i32`.
  println!("2 in array2: {}", array2.into_iter().any(|&x| x == 2));


// pub trait Iterator {
//     // The type being iterated over.
//     type Item;

//     // `find` takes `&mut self` meaning the caller may be borrowed
//     // and modified, but not consumed.
//     fn find<P>(&mut self, predicate: P) -> Option<Self::Item> where
//         // `FnMut` meaning any captured variable may at most be
//         // modified, not consumed. `&Self::Item` states it takes
//         // arguments to the closure by reference.
//         P: FnMut(&Self::Item) -> bool {}
// }
  let vec1 = vec![1, 2, 3];
  let vec2 = vec![4, 5, 6];

  // `iter()` for vecs yields `&i32`.
  let mut iter = vec1.iter();
  // `into_iter()` for vecs yields `i32`.
  let mut into_iter = vec2.into_iter();

  // A reference to what is yielded is `&&i32`. Destructure to `i32`.
  println!("Find 2 in vec1: {:?}", iter     .find(|&&x| x == 2));
  // A reference to what is yielded is `&i32`. Destructure to `i32`.
  println!("Find 2 in vec2: {:?}", into_iter.find(| &x| x == 2));

  let array1 = [1, 2, 3];
  let array2 = [4, 5, 6];

  // `iter()` for arrays yields `&i32`
  println!("Find 2 in array1: {:?}", array1.iter()     .find(|&&x| x == 2));
  // `into_iter()` for arrays unusually yields `&i32`
  println!("Find 2 in array2: {:?}", array2.into_iter().find(|&&x| x == 2));
}

// input params FnOnce
fn apply<F>(f: F) where F: FnOnce() {
  f();
}

// input param Fn
fn apply_to3<F>(f: F) -> i32 where F: Fn(i32) -> i32 {
  f(3)
}

fn create_fn() -> Box<Fn()> {
  let text = "Fn".to_owned();

  Box::new(move || println!("This is a : {}", text))
}

fn create_fnmut() -> Box<FnMut()> {
  let text = "FnMut".to_owned();

  Box::new(move || println!("This is a : {}", text))
}