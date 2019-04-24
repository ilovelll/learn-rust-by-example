fn main() {
    let an_integer = 1;
    let a_boolean = true;
    let unit = ();

    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    let _unused_variable = 3u32;

    // let noisy_unused_variable = 2u32;
    println!("Hello, world!");


    // Scope and Shadowing
    let long_lived_binding = 1;

    {
        let short_lived_binding = 2;

        println!("inner short: {}", short_lived_binding);

        let long_lived_binding = 5_f32;
        
        println!("inner long: {}", long_lived_binding);
    }

    // println!("outer short: {}", short_lived_binding);

    println!("outer long: {}", long_lived_binding);

    let long_lived_binding = 'a';
    println!("outer long: {}", long_lived_binding);

    // Declare a variable binding
    let a_binding;

    {
        let x = 2;
        a_binding = x * x;

    }

    println!("a binding: {}", a_binding);

    let another_binding;
    // println!("another binding: {}", another_binding);

    another_binding = 1;
    println!("another binding: {}", another_binding);
}

