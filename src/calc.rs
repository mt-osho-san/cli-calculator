pub fn add(num1: f64, num2: f64) -> Result<f64, String>{
  Ok(num1+num2)
}

pub fn sub(num1: f64, num2: f64) -> Result<f64, String> {
  Ok(num1 - num2)
}

pub fn mul(num1: f64, num2: f64) -> Result<f64, String> {
  let result = num1 * num2;
  Ok(result)
}

pub fn div(num1: f64, num2: f64) -> Result<f64, String> {
    if num2 == 0.0 {
      Err(String::from("Zero division error"))
    } else {
      Ok(num1/num2)
    }
}