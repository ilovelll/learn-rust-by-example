struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop is being dropped");
    }
}

fn main() {
    println!("Hello, world!");
    let x = ToDrop;
    println!("Made a ToDrop!");

    let immutable_box = Box::new(5u32);
    println!("imutable_box contains {}", immutable_box);

    // *immutable_box = 4;

    let mut mutable_box = immutable_box;
    println!("mutable_box contains {}", mutable_box);

    *mutable_box = 4;
    println!("mutable_box now contains {}", mutable_box);

    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);
    {
        let _ref_to_i32: &i32 = &boxed_i32;
        // Error!
        // Can't destroy `boxed_i32` while the inner value is borrowed later in scope.
        // eat_box_i32(boxed_i32);
        borrow_i32(_ref_to_i32);
    }
    eat_box_i32(boxed_i32);
}

fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("Destroying box that contains {}", boxed_i32);
}

fn borrow_i32(borrowed_i32: &i32) {
    println!("This int is: {}", borrowed_i32);
}