// chapter2/options.rs

fn divide(dividend: u32, divisor: u32) -> Option<u32> {
    if divisor == 0u32 {
        None
    } else {
        Some(dividend / divisor)
    }
}

fn main() {
    let result1 = divide(100, 0);

    match result1 {
        None => println!("Error occured"),
        Some(result) => println!("The result is {}", result),
    }

    let result2 = divide(100, 10);
    println!("The result is {:?}", result2.unwrap());
}
