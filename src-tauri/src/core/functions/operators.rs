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
}

impl Function for Subtraction {
    fn name() -> &'static str {
        "sub"
    }
    fn calc(&self, values: &HashMap<char, f64>) -> f64 {
        self.minuend.eval(values) - self.subtrahend.eval(values)
    }
}

impl Function for Multiplication {
    fn name() -> &'static str {
        "mul"
    }
    fn calc(&self, values: &HashMap<char, f64>) -> f64 {
        self.first_factor.eval(values) - self.second_factor.eval(values)
    }
}

impl Function for Division {
    fn name() -> &'static str {
        "div"
    }
    fn calc(&self, values: &HashMap<char, f64>) -> f64 {
        self.dividend.eval(values) / self.divisor.eval(values)
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
