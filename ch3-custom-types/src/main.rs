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
// enum with implicit discriminator (starts at 0)
enum Number {
    Zero,
    One,
    Two,
}

// enum with explicit discriminator
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        List::Nil
    }

    fn prepend(self, elem: u32) -> List {
        List::Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            List::Cons(_, ref tail) => 1 + tail.len(),
            List::Nil => 0
        }
    }

    fn stringify(&self) -> String {
        match *self {
            List::Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            },
            List::Nil => {
                format!("Nil")
            }
        }
    }
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
    
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);

    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has length: {} ", list.len());
    println!("{}", list.stringify());
    
    let n = 16;
    println!("This is {}", LANG);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });
}

static LANG: &str = "rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}