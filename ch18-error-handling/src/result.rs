use std::num::ParseIntError;
type AliasedResult<T> = Result<T, ParseIntError>;

fn multiply(first_number_str: &str, second_number_str: &str) -> i32 {
  let first_number = first_number_str.parse::<i32>().unwrap();
  let second_number = second_number_str.parse::<i32>().unwrap();
  first_number * second_number
}

fn multiply_v2(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
  match first_number_str.parse::<i32>() {
    Ok(first_number) => {
      match second_number_str.parse::<i32>() {
        Ok(second_number) => {
          Ok(first_number * second_number)
        },
        Err(e) => Err(e),
      }
    },
    Err(e) => Err(e),
  }
}

fn multiply_v3(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
  first_number_str.parse::<i32>().and_then(|first_number| {
    second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
  })
}

fn multiply_v4(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
  let first_number = first_number_str.parse::<i32>()?;
  let second_number = second_number_str.parse::<i32>()?;

  Ok(first_number * second_number)
}

fn print(result: AliasedResult<i32>) {
  match result {
    Ok(n)  => println!("n is {}", n),
    Err(e) => println!("Error: {}", e),
  }
}
fn main() {
  let twenty = multiply("10", "2");
  println!("double is {}", twenty);

  let tt = multiply_v2("tt", "2");
  print(tt);

  let tt = multiply_v3("tt", "2");
  print(tt);

  let tt = multiply_v4("tt", "2");
  print(tt);
}