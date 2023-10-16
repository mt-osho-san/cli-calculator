use std::{io::Result};

pub fn add(num1: f64, num2: f64) -> Result<f64>{
  let res = num1 + num2;
  Ok(res)
}

pub fn sub(num1: f64, num2: f64) -> Result<f64> {
  let result = num1 - num2;
  Ok(result)
}

pub fn mul(num1: f64, num2: f64) -> Result<f64> {
  let result = num1 * num2;
  Ok(result)
}

pub fn div(num1: f64, num2: f64) -> Result<f64> {
    let result = num1 / num2;
    Ok(result)
}