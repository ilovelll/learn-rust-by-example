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
}
