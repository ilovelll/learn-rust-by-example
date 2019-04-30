fn is_odd(n: u32) -> bool {
  n % 2 == 1
}

fn main() {
  println!("Find the sum of all the squared odd umbers under 1000");
  let upper = 1000;

  let mut acc = 0;
  for n in 0.. {
    let n_squared = n * n;

    if n_squared >= upper {
      break;
    } else if is_odd(n_squared) {
      acc += n_squared;
    }
  }
  println!("imperative style: {}", acc);

  let sum_of_squared_odd_numbers: u32 =
    (0..).map(|n| n*n)
          .take_while(|&n_squared| n_squared < upper)
          .filter(|&n_squared| is_odd(n_squared))
          .fold(0, |acc, n_squared| acc+ n_squared);
  println!("function style: {}", sum_of_squared_odd_numbers);


  // The main advantage of `!` never type is that it can be cast to any other one 
  // and therefore used at places where an exact type is required, 
  // for instance in match branches. 
  fn sum_odd_numbers(up_to: u32) -> u32 {
    let mut acc = 0;
    for i in 0..up_to {
      let addition: u32 = match i%2 == 1 {
        true => i,
        // On the other hand, the "continue" expression does not return
        // u32, but it is still fine, because it never returns and therefore
        // does not violate the type requirements of the match expression.
        false => continue,
      };
      acc += addition;
    }
    acc
  }

  println!("Sum of odd numbers up to 9 (excluding): {}", sum_odd_numbers(9));
}