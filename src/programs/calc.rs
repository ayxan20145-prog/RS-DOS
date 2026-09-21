use crate::{drivers::vga::writer, print};
use core::fmt::Write;

pub fn calc(line: &str) {
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
                    print!("\n{}", num1 + num2);
                }
                "-" => {
                    print!("\n{}", num1 - num2);
                }
                "*" => {
                    print!("\n{}", num1 * num2);
                }
                "/" => {
                    print!("\n{}", num1 / num2);
                }
                _ => {
                    print!("\nunknown operator: {}", op);
                }
            };
        }
        _ => {
            print!("\n");
        }
    }
}
