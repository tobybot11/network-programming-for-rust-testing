// chapter2 / syntactic-macro.rs

#![feature(trace_macros)]

macro_rules! factorial {
    ($x:expr) => {{
        // (1..$x + 1).fold(1, |acc, x| acc * x)
        let mut result = 1;
        for i in 1..($x + 1) {
            // result *= i;
            result = result * i;
        }
        result
    }};
}

fn main() {
    let arg = std::env::args()
        .nth(1)
        .expect("Please provide only 1 argument");

    trace_macros!(true);
    println!(
        "The factorial of {} is {:?}",
        arg,
        factorial!(arg.parse::<u64>().expect("Could not parse to an integer"))
    );
    trace_macros!(false);
}
