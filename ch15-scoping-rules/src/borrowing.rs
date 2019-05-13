#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
  author: &'static str,
  title: &'static str,
  year: u32,
}

fn borrow_book(book: &Book) {
  println!("I immutably borrowed {} - {} edition", book.title, book.year);
}

fn new_edition(book: &mut Book) {
  book.year = 2014;
  println!("I mutably borrowed {} - {}", book.title, book.year);
}

#[derive(Clone, Copy)]
struct Point {x: i32, y: i32, z: i32}

fn main() {
  let immutabook = Book {
    author: "Douglas Hofstadter",
    title: "Gödel, Escher, Bach",
    year: 1979,
  };

  let mut mutabook = immutabook;

  borrow_book(&immutabook);
  borrow_book(&mutabook);

  new_edition(&mut mutabook);
  // new_edition(&mut immutabook);

  let mut _mutable_integer = 7i32;
  // only one mutable borrow or any number immutable borrow
  // is allowed at a time
  {
    let _large_integer = &_mutable_integer;

    // Error! `_mutable_integer` is fronzen in this scope
    // _mutable_integer = 50;
    
    println!("mutable {}", _large_integer);
  }
  _mutable_integer = 4;

    let mut point = Point { x: 0, y: 0, z: 0 };

    {
        let borrowed_point = &point;
        let another_borrow = &point;

        // Data can be accessed via the references and the original owner
        println!("Point has coordinates: ({}, {}, {})",
                 borrowed_point.x, another_borrow.y, point.z);

        // will not cause Error because NLL
        // borrowed as immutable.
        let mutable_borrow = &mut point;
        // TODO ^ Try uncommenting this line

        // Immutable references go out of scope
    }

    {
        let mutable_borrow = &mut point;

        // Change data via mutable reference
        mutable_borrow.x = 5;
        mutable_borrow.y = 2;
        mutable_borrow.z = 1;

        // Error! Can't borrow `point` as immutable because it's currently
        // borrowed as mutable.
        // let y = &point.y;
        // TODO ^ Try uncommenting this line

        // Error! Can't print because `println!` takes an immutable reference.
        // println!("Point Z coordinate is {}", point.z);
        // TODO ^ Try uncommenting this line

        // Ok! Mutable references can be passed as immutable to `println!`
        println!("Point has coordinates: ({}, {}, {})",
                 mutable_borrow.x, mutable_borrow.y, mutable_borrow.z);

        // Mutable reference goes out of scope
    }

    let borrow_point = &point;
    println!("Point now has coordinates: ({}, {}, {})", borrow_point.x, borrow_point.y, borrow_point.z);

    println!("==============================");

  let c = 'Q';

  let ref ref_c1 = c;
  let ref_c2 = &c;

  println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);

  let point = Point {x: 0, y: 0, z: 0};

  let _copy_of_x = {
    let Point {x: ref ref_to_x, ..} = point;
    *ref_to_x
  };

  let mut mutable_point = point;
  {
    // `ref` can be paired with `mut` to take mutable references.
    let Point { x: _, y: ref mut mut_ref_to_y, z: _} = mutable_point;

    *mut_ref_to_y = 1;
  }

  println!("point is ({}, {}, {})", point.x, point.y, point.z);
  println!("mutable_point is ({}, {}, {})", mutable_point.x, mutable_point.y, mutable_point.z);

  let mut mutable_tuple = (Box::new(5u32), 3u32);
  {
    // Destructure `mutable_tuple` to change the value of `last`.
    let (_, ref mut last) = mutable_tuple;
    *last = 2u32;
  }

  println!("tuple is {:?}", mutable_tuple);
}