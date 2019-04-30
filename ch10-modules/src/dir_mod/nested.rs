pub fn function() {
    println!("called `dir_mod::nested::function()`");
}

#[allow(dead_code)]
fn private_function() {
    println!("called `dir_mod::nested::private_function()`");
}