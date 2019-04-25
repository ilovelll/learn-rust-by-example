fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // return value with break
        }
    };

    assert_eq!(result, 20);

    let mut n = 1;

    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        n += 1;
    }

    // a..b or a..=b
    for n in 1..=100 {
        match n {
            // `if condition` part is a guard
            x if x % 15 ==0 => println!("fizzbuzz"),
            x if x % 3 == 0 => println!("fizz"),
            x if x % 5 == 0 => println!("buzz"),
            _ => println!("{}", n),
        }
    }

    let names = vec!["Bob", "Alice", "Frank"];

    // `iter` borrow each element of the collection throgh each iteration
    // `names` untouched and available for reuse after the loop
    for name in names.iter() {
        match name {
            &"Frank" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    println!("{:#?}", names);

    // `into_iter` consumes the collection, not borrow
    // `names` no longer live
    for name in names.into_iter() {
        match name {
            "Frank" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    // should comment below, `names` has been move
    // println!("{:#?}", names);

    let mut names = vec!["Alice", "Bob", "Frank"];
    // `iter_mut` can modified the value
    for name in names.iter_mut() {
        match name {
            &mut "Frank" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    println!("{:#?}", names);


    // match can destruct enums,pointers,structures,tuples

    // match tuple
    let pair = (0, -2);
    println!("Tell me about {:?}", pair);

    match pair {
        (0, y) => println!("First is `0` and `y` is `{:?}`", y),
        (x, 0) => println!("`x` is {:?} and last is `0`", x),
        _ => println!("It doesn't matter what they are"),
    }

    //match enums
    // `allow` required to silence warnings because only
    // one variant is used.
    #[allow(dead_code)]
    enum Color {
        // These 3 are specified solely by their name.
        Red,
        Blue,
        Green,
        // These likewise tie `u32` tuples to different names: color models.
        RGB(u32, u32, u32),
        HSV(u32, u32, u32),
        HSL(u32, u32, u32),
        CMY(u32, u32, u32),
        CMYK(u32, u32, u32, u32),
    }

    let color = Color::RGB(122, 17, 40);
    // TODO ^ Try different variants for `color`

    println!("What color is it?");
    // An `enum` can be destructured using a `match`.
    match color {
        Color::Red   => println!("The color is Red!"),
        Color::Blue  => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) =>
            println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) =>
            println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) =>
            println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) =>
            println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) =>
            println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
                c, m, y, k),
        // Don't need another arm because all variants have been examined
    }


    // match ref

    // a reference
    let reference = &4;
    println!("{}", *reference);

    // desctructuring
    match reference {
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    // dereferencing
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    let _not_a_reference = 4;

    let ref _is_a_reference = 3;

    let value = 5;
    let mut mut_value = 6;

    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    match mut_value {
        ref mut m => {
            *m += 10;
            println!("Added 10. `mut_value`: {:?}", m);
        },
    }

    struct Foo { x: (u32, u32), y: u32 }

    // destructure members of the struct
    let foo = Foo { x: (1, 2), y: 3 };
    let Foo { x: (a, b), y } = foo;

    println!("a = {}, b = {},  y = {} ", a, b, y);

    // you can destructure structs and rename the variables,
    // the order is not important

    let Foo { y: i, x: j } = foo;
    println!("i = {:?}, j = {:?}", i, j);

    // and you can also ignore some variables:
    let Foo { y, .. } = foo;
    println!("y = {}", y);

    // this will give an error: pattern does not mention field `x`
    // let Foo { y } = foo;

    // Binding at match use `@`
    match age() {
        0            => println!("I'm not born yet I guess"),
        n @ 1 ... 12 => println!("I'm a child of age {:?}", n),
        n @ 13... 19 => println!("Im a teen of age {:?}", n),
        n            => println!("I'm a old person of age {:?}", n),
    }
}

fn age() -> u32 {
    15
}
