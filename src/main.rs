use structopt::StructOpt;
mod cli;
mod calc;

use cli::{Operation::*, CommandLineArgs};

fn main() {
    let CommandLineArgs {
        operation,
    } = CommandLineArgs::from_args();
    
    let x = match operation {
        Add { num1, num2 } => calc::add(num1, num2),
        Sub { num1, num2 } => calc::sub(num1, num2),
        Mul { num1, num2 } => calc::mul(num1, num2),
        Div { num1, num2 } => calc::div(num1, num2),
    }
    .expect("failed to operate");

    println!("The ansewer is {}", x);
}
