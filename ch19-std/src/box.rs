use std::mem;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct Point {
  x: f64,
  y: f64,
}

#[allow(dead_code)]
struct Rectangle {
  p1: Point,
  p2: Point,
}

fn origin() -> Point{
  Point{
    x: 0.0,
    y: 0.0,
  }
}

fn boxed_origin() -> Box<Point> {
  Box::new(Point { x: 0.0, y: 0.0 })
}

fn main() {
  let point = origin();
  let rectangle = Rectangle {
    p1: origin(),
    p2: Point { x: 3.0, y: 4.0 }
  };

  let boxed_rectangle = Box::new(Rectangle {
    p1: origin(),
    p2: origin(),
  });

  let boxed_point = Box::new(origin());

  let boxed_in_a_box = Box::new(boxed_origin());

  println!("Point occupies {} bytes in the stack", mem::size_of_val(&point));
  println!("Rectangle occupies {} bytes in the stack", mem::size_of_val(&rectangle));

  // box size = pointer size
  println!("Boxed point occupies {} bytes in the statck", mem::size_of_val(&boxed_point));
  println!("Boxed rectangle occupies {} bytes in the statck", mem::size_of_val(&boxed_rectangle));
  println!("Boxed box occupies {} bytes in the statck", mem::size_of_val(&boxed_in_a_box));


  let unboxed_point = *boxed_point;
  println!("Unboxed point occupies {} bytes in the stack", mem::size_of_val(&unboxed_point));
}