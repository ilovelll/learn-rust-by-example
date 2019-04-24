use std::convert::From;
use std::string::ToString;
#[derive(Debug)]
struct Number {
    value: i32,
}
struct Circle {
    radius: i32
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number {value: item}
    }
}

impl ToString for Circle {
    fn to_string(&self) -> String {
        format!("Circle of radius {:?}", self.radius)
    }
}


fn main() {
    let num = Number::from(30);
    let int = 5;
    let _num: Number = int.into();
    println!("My number is {:?}", _num);
    println!("My number is {:?}", num);

    let cicle = Circle { radius: 4};
    println!("{}", cicle.to_string());
}