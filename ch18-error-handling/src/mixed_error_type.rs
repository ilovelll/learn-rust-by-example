use std::num::ParseIntError;
use std::error;
use std::fmt;

type IResult<T> = std::result::Result<T, DoubleError>;
type BResult<T> = std::result::Result<T, Box<error::Error>>;
type WResult<T> = std::result::Result<T, WrapError>;
#[derive(Debug, Clone)]
struct DoubleError;

#[derive(Debug, Clone)]
struct EmptyVec;

impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for DoubleError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for EmptyVec {
    fn description(&self) -> &str {
        "invalid first item to double"
    }
    
    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

#[derive(Debug)]
enum WrapError {
    EmptyVec,
    Parse(ParseIntError)
}

impl fmt::Display for WrapError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WrapError::EmptyVec =>
                write!(f, "please use a vector with at least one element"),
            WrapError::Parse(ref e) => e.fmt(f),
        }
    }
}

impl error::Error for WrapError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            WrapError::EmptyVec => None,
            WrapError::Parse(ref e) => Some(e),
        }
    }
}

impl From<ParseIntError> for WrapError {
    fn from(err: ParseIntError) -> WrapError {
        WrapError::Parse(err)
    }
}

// embed Result with Option
fn double_first(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
    vec.first().map(|first| {
        first.parse::<i32>().map(|n| 2 * n)
    })
}

fn double_first_v2(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
    let opt = vec.first().map(|first| {
        first.parse::<i32>().map(|n| 2 * n)
    });

    let opt = opt.map_or(Ok(None), |r| r.map(Some))?;
    Ok(opt)
}

fn double_first_v3(vec: Vec<&str>) -> IResult<i32> {
    vec.first()
       .ok_or(DoubleError)
       .and_then(|s| {
           s.parse::<i32>()
                .map_err(|_| DoubleError)
                .map(|i| 2 * i)
       })
}

fn double_first_v4(vec: Vec<&str>) -> BResult<i32> {
    vec.first()
        .ok_or_else(|| EmptyVec.into())
        .and_then(|s| {
            s.parse::<i32>()
                .map_err(|e| e.into())
                .map(|i| 2 * i)
        })
}

fn double_first_v5(vec: Vec<&str>) -> BResult<i32> {
    let first = vec.first().ok_or(EmptyVec)?;
    let parsed = first.parse::<i32>()?;
    Ok(2 * parsed)
}

fn double_first_v6(vec: Vec<&str>) -> WResult<i32> {
    let first = vec.first().ok_or(WrapError::EmptyVec)?;
    let parsed = first.parse::<i32>()?;

    Ok(2 * parsed)
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("The first doubled is {:?}", double_first(numbers.to_vec()));

    println!("The first doubled is {:?}", double_first(empty.to_vec()));
    // Error 1: the input vector is empty

    println!("The first doubled is {:?}", double_first(strings.to_vec()));
    // Error 2: the element doesn't parse to a number

    println!("**************************");    

    println!("2 The first doubled is {:?}", double_first_v2(numbers.to_vec()));

    println!("2 The first doubled is {:?}", double_first_v2(empty.to_vec()));
    // Error 1: the input vector is empty

    println!("2 The first doubled is {:?}", double_first_v2(strings.to_vec()));
    // Error 2: the element doesn't parse to a number

    println!("**************************");    

    println!("3 The first doubled is {:?}", double_first_v3(numbers.to_vec()));

    println!("3 The first doubled is {:?}", double_first_v3(empty.to_vec()));
    // Error 1: the input vector is empty

    println!("3 The first doubled is {:?}", double_first_v3(strings.to_vec()));
    // Error 2: the element doesn't parse to a number


    println!("**************************");    

    println!("4 The first doubled is {:?}", double_first_v4(numbers.to_vec()));

    println!("4 The first doubled is {:?}", double_first_v4(empty.to_vec()));
    // Error 1: the input vector is empty

    println!("4 The first doubled is {:?}", double_first_v4(strings.to_vec()));
    // Error 2: the element doesn't parse to a number

    println!("**************************");    

    println!("5 The first doubled is {:?}", double_first_v5(numbers.to_vec()));

    println!("5 The first doubled is {:?}", double_first_v5(empty.to_vec()));
    // Error 1: the input vector is empty

    println!("5 The first doubled is {:?}", double_first_v5(strings.to_vec()));
    // Error 2: the element doesn't parse to a number

    println!("**************************");    

    println!("6 The first doubled is {:?}", double_first_v6(numbers.to_vec()));

    println!("6 The first doubled is {:?}", double_first_v6(empty.to_vec()));
    // Error 1: the input vector is empty

    println!("6 The first doubled is {:?}", double_first_v6(strings.to_vec()));
    // Error 2: the element doesn't parse to a numbe
}
