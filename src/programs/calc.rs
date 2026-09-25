pub fn calc(line: &str) -> Result<f64, &str> {
    let mut parts = line.split_whitespace();

    parts.next();

    let num1 = parts.next();
    let op = parts.next();
    let num2 = parts.next();

    match (num1, op, num2) {
        (Some(num1), Some(op), Some(num2)) => {
            let num1 = num1.parse::<f64>().unwrap();
            let num2 = num2.parse::<f64>().unwrap();

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
                        return Err("cant divide by zero");
                    } else {
                        return Ok(num1 / num2);
                    }
                }
                _ => {
                    return Err("unknown operator");
                }
            };
        }
        _ => {
            return Err("");
        }
    }
}
