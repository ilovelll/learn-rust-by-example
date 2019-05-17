use std::ops;
#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;
        Centimeters(inches as f64 * 2.54)
    }
}

#[derive(Debug)]
struct Seconds(i32);

impl ops::Add<Seconds> for Seconds {
    type Output = Seconds;

    fn add(self, rhs: Seconds) -> Seconds {
        println!("> Seconds.add(Seconds) was calle1d");
        Seconds(self.0 + rhs.0)
    }
}

impl Drop for Seconds {
    fn drop(&mut self) {
        println!("> Dropping {}", self.0);
    }
}

fn main() {
    let _one_second = Seconds(1);
    let two_seconds = Seconds(2);
    
    println!("1 second add 2 seconds is {:?}", _one_second + two_seconds);

    // let _this_is_true = (_one_second == __one_second);

    let foot = Inches(12);

    println!("One foot equals {:?}", foot);

    let meter = Centimeters(100.0);
    let cmp = if foot.to_centimeters() < meter {
        "smaller"
    } else {
        "bigger"
    };
    println!("One foot is {} than one meter", cmp);
}
