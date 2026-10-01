use crate::std::error::{Error, Result};

pub fn calc(line: &str) -> Result<f64> {
    let mut parts = line.split_whitespace();

    parts.next();

    let num1 = parts.next();
    let op = parts.next();
    let num2 = parts.next();

    match (num1, op, num2) {
        (Some(num1), Some(op), Some(num2)) => {
            let num1 = num1.parse::<f64>().map_err(|_| Error::InvalidOperand)?;
            let num2 = num2.parse::<f64>().map_err(|_| Error::InvalidOperand)?;

            match op {
                "+" => {
                    return Ok(num1 + num2);
                }
                "-" => {
                    return Ok(num1 - num2);
                }
                "*" => {
                    return Ok(num1 * num2);
                }
                "/" => {
                    if num2 == 0.0 {
                        return Err(Error::DivisionByZero);
                    } else {
                        return Ok(num1 / num2);
                    }
                }
                _ => {
                    return Err(Error::UnknownOperator);
                }
            };
        }
        _ => {
            return Err(Error::MissingArgument);
        }
    }
}
