use structopt::StructOpt;
mod cli;
mod calc;

use cli::{Operation::*, CommandLineArgs};

fn main() {
    let CommandLineArgs {
        operation,
    } = CommandLineArgs::from_args();
    
    let result = match operation {
        Add { num1, num2 } => calc::add(num1, num2),
        Sub { num1, num2 } => calc::sub(num1, num2),
        Mul { num1, num2 } => calc::mul(num1, num2),
        Div { num1, num2 } => calc::div(num1, num2),
    };

    match result {
        Ok(v) => println!("The answer is {}", v),
        Err(e) => eprintln!("Error: {}", e),
    }
}

#[test]
fn add_two_numbers() {
    let result = calc::add(2_f64,3_f64);
    assert_eq!(result, Ok(5_f64));
}

#[test]
fn sub_two_numbers() {
    let result = calc::sub(3.0, 2.0);
    assert_eq!(result, Ok(1.0));
}

#[test]
fn mul_two_numbers() {
    let result = calc::mul(3.0, 2.0);
    assert_eq!(result, Ok(6.0));
}

#[test]
fn div_two_numbers() {
    let result = calc::div(3.0, 2.0);
    assert_eq!(result, Ok(1.5));
}