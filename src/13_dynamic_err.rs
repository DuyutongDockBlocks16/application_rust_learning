use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
struct DivisionByZeroError;

// Display is required to implement the Error trait
impl Display for DivisionByZeroError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "division by zero")
    }
}

impl Error for DivisionByZeroError {}

fn divide_str_ints(x: &str, y: &str) -> Result<i32, Box<dyn Error>> {
    let x = x.trim().parse::<i32>()?;
    let y = y.trim().parse::<i32>()?;
    if y == 0 {
        return Err(Box::new(DivisionByZeroError));
    }
    Ok(x / y)
}


fn main() {
    let x = "10";
    let y = "0";
    match divide_str_ints(x, y) {
        Ok(v) => println!("{} / {} = {}", x, y, v),
        Err(e) => println!("Error: {}", e),
    }
}
