use std::collections::HashMap;

// Basic trait for any math expression
pub enum ExpressionType<'a> {
    Const {
        value: f64,
    },
    Variable {
        identifier: char,
    },
    Sum {
        first_addend: &'a Expression<'a>,
        second_addend: &'a Expression<'a>,
    },
    Subtraction {
        minuend: &'a Expression<'a>,
        subtrahend: &'a Expression<'a>,
    },
    Multiplication {
        first_factor: &'a Expression<'a>,
        second_factor: &'a Expression<'a>,
    },
    Division {
        dividend: &'a Expression<'a>,
        divisor: &'a Expression<'a>,
    },
}

pub struct Expression<'a> {
    pub r#type: ExpressionType<'a>,
}

impl Expression<'_> {
    fn eval(&self, values: &HashMap<char, f64>) -> f64 {
        match self.r#type {
            ExpressionType::Const { value } => value,
            ExpressionType::Variable { identifier } => *values
                .get(&identifier)
                .expect(&format!("No value provided for: '{}'", identifier)),
            ExpressionType::Sum {
                first_addend,
                second_addend,
            } => first_addend.eval(values) + second_addend.eval(values),
            ExpressionType::Subtraction {
                minuend,
                subtrahend,
            } => minuend.eval(values) - subtrahend.eval(values),
            ExpressionType::Multiplication {
                first_factor,
                second_factor,
            } => first_factor.eval(values) * second_factor.eval(values),
            ExpressionType::Division { dividend, divisor } => {
                dividend.eval(values) / divisor.eval(values)
            }
        }
    }
}
