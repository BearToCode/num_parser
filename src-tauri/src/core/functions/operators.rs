use std::collections::HashMap;

use super::{Expression, Function};
use serde::{Deserialize, Serialize};

// Structs
#[derive(Serialize, Deserialize)]
pub struct Sum {
    first_addend: Box<Expression>,
    second_addend: Box<Expression>,
}

#[derive(Serialize, Deserialize)]
pub struct Subtraction {
    minuend: Box<Expression>,
    subtrahend: Box<Expression>,
}

#[derive(Serialize, Deserialize)]
pub struct Multiplication {
    first_factor: Box<Expression>,
    second_factor: Box<Expression>,
}

#[derive(Serialize, Deserialize)]
pub struct Division {
    dividend: Box<Expression>,
    divisor: Box<Expression>,
}

// Implementation
impl Function for Sum {
    fn name() -> &'static str {
        "sum"
    }
    fn calc(&self, values: &HashMap<char, f64>) -> f64 {
        self.first_addend.eval(values) + self.second_addend.eval(values)
    }
    fn build(mut arguments: Vec<Expression>) -> Result<Box<Self>, String> {
        if arguments.len() != 2 {
            Err(super::invalid_arguments_count_err(
                Sum::name(),
                2,
                arguments.len(),
            ))
        } else {
            Ok(Box::new(Sum {
                second_addend: Box::new(arguments.pop().unwrap()),
                first_addend: Box::new(arguments.pop().unwrap()),
            }))
        }
    }
}

impl Function for Subtraction {
    fn name() -> &'static str {
        "sub"
    }
    fn calc(&self, values: &HashMap<char, f64>) -> f64 {
        self.minuend.eval(values) - self.subtrahend.eval(values)
    }
    fn build(mut arguments: Vec<Expression>) -> Result<Box<Self>, String> {
        if arguments.len() != 2 {
            Err(super::invalid_arguments_count_err(
                Sum::name(),
                2,
                arguments.len(),
            ))
        } else {
            Ok(Box::new(Subtraction {
                subtrahend: Box::new(arguments.pop().unwrap()),
                minuend: Box::new(arguments.pop().unwrap()),
            }))
        }
    }
}

impl Function for Multiplication {
    fn name() -> &'static str {
        "mul"
    }
    fn calc(&self, values: &HashMap<char, f64>) -> f64 {
        self.first_factor.eval(values) * self.second_factor.eval(values)
    }
    fn build(mut arguments: Vec<Expression>) -> Result<Box<Self>, String> {
        if arguments.len() != 2 {
            Err(super::invalid_arguments_count_err(
                Sum::name(),
                2,
                arguments.len(),
            ))
        } else {
            Ok(Box::new(Multiplication {
                second_factor: Box::new(arguments.pop().unwrap()),
                first_factor: Box::new(arguments.pop().unwrap()),
            }))
        }
    }
}

impl Function for Division {
    fn name() -> &'static str {
        "div"
    }
    fn calc(&self, values: &HashMap<char, f64>) -> f64 {
        self.dividend.eval(values) / self.divisor.eval(values)
    }
    fn build(mut arguments: Vec<Expression>) -> Result<Box<Self>, String> {
        if arguments.len() != 2 {
            Err(super::invalid_arguments_count_err(
                Sum::name(),
                2,
                arguments.len(),
            ))
        } else {
            Ok(Box::new(Division {
                divisor: Box::new(arguments.pop().unwrap()),
                dividend: Box::new(arguments.pop().unwrap()),
            }))
        }
    }
}

pub fn operator_data_from_char(c: &char) -> Option<(&'static str, u8)> {
    match c {
        '+' => Some((Sum::name(), 1)),
        '-' => Some((Subtraction::name(), 1)),
        '*' => Some((Multiplication::name(), 2)),
        '/' => Some((Division::name(), 3)),
        _ => None,
    }
}
