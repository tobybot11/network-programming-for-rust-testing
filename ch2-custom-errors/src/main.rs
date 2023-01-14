// chapter2/custom-errors.rs

use std::fmt;
use std::error::Error;

#[derive(Debug)]
enum OperationsError {
    DivideByZeroError,
}

// Useful for displaying the error nicely
impl fmt::Display for OperationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            OperationsError::DivideByZeroError => f.write_str("Cannot divide by zero motherfucker"),
        }
    }
}
// Registers the custom error as an error
impl Error for OperationsError {
    fn description(&self) -> &str {
        match *self {
            OperationsError::DivideByZeroError => "Cannot divide by zero",
        }
    }
}

// Divides the dividend by the divisor and returns the results. returns
// an error if the divisor is zero
fn divide(dividend: u32, divisor: u32) -> Result<u32, OperationsError> {
    if divisor == 0u32 {
        Err(OperationsError::DivideByZeroError)
    } else {
        Ok(dividend / divisor)
    }
}

fn main() {
    let result1 = divide(100, 0);
    println!("Result1: {:?}", result1);

    let result2 = divide(100, 2);
    println!("Result2: {:?}", result2);
    println!("Result2: {:?}", result2.unwrap());
}
