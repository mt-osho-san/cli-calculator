use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Operation {
  Add {
    num1: f64,
    num2: f64,
  },
  Sub {
    num1: f64,
    num2: f64,
  },
  Mul {
    num1: f64,
    num2: f64,
  },
  Div {
    num1: f64,
    num2: f64,
  }
}

#[derive(Debug, StructOpt)]
#[structopt(
  name = "Rust Calculator",
  about = "this is a simple cli calculator"
)]
pub struct CommandLineArgs {
  #[structopt(subcommand)]
  pub operation: Operation,
}