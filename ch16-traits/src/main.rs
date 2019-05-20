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

struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;
    fn next(&mut self) ->Option<u32> {
        let new_next = self.curr + self.next;

        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 1, next: 1}
}

#[derive(Clone, Debug)]
struct Pair(Box<i32>, Box<i32>);

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

    println!("======================================");

    let mut sequence = 0..3;

    println!("Four consecutive `next` calls on 0..3");
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());

    println!("Iterate through 0..3 using `for`");
    for i in 0..3 {
        println!("> {}", i);
    }
    println!("The next four terms of the Fibonacci sequence are: ");
    for i in fibonacci().skip(4).take(4) {
        println!("> {}", i);
    }

    println!("======================================");

    let pair = Pair(Box::new(1), Box::new(2));
    println!("original: {:?}", pair);

    let moved_pair = pair;
    println!("copy: {:?}", moved_pair);
    // println!("original: {:?}", pair);

    let cloned_pair = moved_pair.clone();
    println!("copy: {:?}", moved_pair);
    drop(moved_pair);

    println!("clone: {:?}", cloned_pair);
}
