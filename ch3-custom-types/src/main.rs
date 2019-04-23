#[derive(Debug)]
struct Person<'a> {
  name: &'a str,
  age: u8,
}

struct Nil;

struct Pair(i32, f32);
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn rect_area(rec: Rectangle) -> f32 {
    let Point {x: x1, y:y1} = rec.p1;
    let Point {x: x2, y:y2} = rec.p2;

    (x1-x2) * (y1-y2)
}

fn square(point: Point, width: f32) -> Rectangle {
    let Point {x: mx, y: my } = point;
    let new_point = Point { x: mx + width, y: my + width };
    Rectangle{ p1: point, p2: new_point }
}

fn main() {

    let name = "Perter";
    let age = 27;
    let peter = Person { name, age};
    println!("{:?}", peter);

    let point: Point = Point { x: 0.3, y: 0.4 };

    println!("point coordinate: ({} {})", point.x, point.y);

    let new_point = Point{ x: 0.1, ..point};
    println!("second point: ({}, {})", new_point.x, new_point.y);

    let Point { x: my_x, y: my_y } = point;

    let _rectangle = Rectangle {
        p1: Point { x: my_x, y: my_y },
        p2: point,
    };

    let _nil = Nil;
    let pair = Pair(1, 0.1);
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);
    let rec = Rectangle {
        p1: Point { x: 1.0, y: 2.0 },
        p2: Point { x: 2.0, y: 4.0 },
    };
    println!("{}", rect_area(rec));

    println!("{:?}", square(Point { x: 1.0, y: 2.0 }, 2.0));
}
