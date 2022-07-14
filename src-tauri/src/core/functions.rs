pub mod operators;
pub mod trigonometry;

use std::collections::HashMap;

pub enum ExpressionType {
    Value,
    Operator {
        priority: u16,
        identifier: char,
        name: &'static str,
    },
    Function {
        name: &'static str,
    },
}

pub enum Expression<'a> {
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

pub fn variant_eq(a: &ExpressionType, b: &ExpressionType) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}

impl Expression<'_> {
    pub fn r#type(&self) -> ExpressionType {
        match self {
            Expression::Const { value } => ExpressionType::Value,
            Expression::Variable { identifier } => ExpressionType::Value,
            Expression::Sum {
                first_addend,
                second_addend,
            } => ExpressionType::Operator {
                priority: 1,
                identifier: '+',
                name: "sum",
            },
            Expression::Subtraction {
                minuend,
                subtrahend,
            } => ExpressionType::Operator {
                priority: 1,
                identifier: '-',
                name: "sub",
            },
            Expression::Multiplication {
                first_factor,
                second_factor,
            } => ExpressionType::Operator {
                priority: 2,
                identifier: '*',
                name: "mul",
            },
            Expression::Division { dividend, divisor } => ExpressionType::Operator {
                priority: 3,
                identifier: '/',
                name: "div",
            },
        }
    }

    pub fn eval(&self, values: &HashMap<char, f64>) -> f64 {
        match self {
            Expression::Const { value } => *value,
            Expression::Variable { identifier } => *values.get(identifier).unwrap(),
            Expression::Sum {
                first_addend,
                second_addend,
            } => self::operators::sum(first_addend, second_addend, values),
            Expression::Subtraction {
                minuend,
                subtrahend,
            } => self::operators::sub(minuend, subtrahend, values),
            Expression::Multiplication {
                first_factor,
                second_factor,
            } => self::operators::mul(first_factor, second_factor, values),
            Expression::Division { dividend, divisor } => {
                self::operators::div(dividend, divisor, values)
            }
        }
    }
}
