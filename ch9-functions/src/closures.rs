fn closures() {
  fn function (i: i32) -> i32 { i + 1}

  let closure_annotated = |i: i32| -> i32 {i + 1};
  let closure_inferred = |i| i + 1;

  let i = 1;
  println!("function: {}", function(1));
  println!("closure_anotated: {}", closure_annotated(1));
  println!("closure_inferred: {}", closure_inferred(1));

  let one = || 1;
  println!("closure returing one: {}", one());
}