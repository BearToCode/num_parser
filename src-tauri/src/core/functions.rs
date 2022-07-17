pub mod operators;
pub mod trigonometry;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use self::operators::{Division, Multiplication, Subtraction, Sum};

pub trait Function {
    fn name() -> &'static str;
    fn calc(&self, values: &HashMap<char, f64>) -> f64;
    fn build(arguments: Vec<Expression>) -> Result<Box<Self>, String>;
}

pub fn invalid_arguments_count_err(function_name: &str, expected: u8, given: usize) -> String {
    String::from(format!(
        "Invalid number of arguments for function '{}': {} were expected but {} were given!",
        function_name, expected, given
    ))
}

#[derive(Serialize, Deserialize)]
pub enum Expression {
    Const { value: f64 },
    Variable { identifier: char },
    Sum(Sum),
    Subtraction(Subtraction),
    Multiplication(Multiplication),
    Division(Division),
}

impl Expression {
    pub fn eval(&self, values: &HashMap<char, f64>) -> f64 {
        match self {
            Expression::Const { value } => *value,
            Expression::Variable { identifier } => *values.get(identifier).unwrap(),
            Expression::Sum(sum) => sum.calc(values),
            Expression::Subtraction(subtraction) => subtraction.calc(values),
            Expression::Multiplication(multiplication) => multiplication.calc(values),
            Expression::Division(division) => division.calc(values),
        }
    }
}
