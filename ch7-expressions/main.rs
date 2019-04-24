fn main() {
  // statement: variable binding 
  let x = 5u32;

  // statement:  expression with ';'
  x + 1;

  // block as r-values
  let y = {
    let x_squared = x*x;
    let x_cube = x_squared + x;

    x_cube + x_squared + x
  };

  let z = {
    // The `;` semicolon suppresses this expression and `()` is assigned to `z`
    2 * x;
  };

  println!("x is {:?}", x);
  println!("y is {:?}", y);
  println!("z is {:?}", z);
}