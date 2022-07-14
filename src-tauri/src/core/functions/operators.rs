use std::collections::HashMap;

use super::Expression;

pub fn sum(
    first_addend: &Expression,
    second_addend: &Expression,
    values: &HashMap<char, f64>,
) -> f64 {
    first_addend.eval(values)
}

pub fn sub(minuend: &Expression, subtrahend: &Expression, values: &HashMap<char, f64>) -> f64 {
    minuend.eval(values) - subtrahend.eval(values)
}

pub fn mul(
    first_factor: &Expression,
    second_factor: &Expression,
    values: &HashMap<char, f64>,
) -> f64 {
    first_factor.eval(values) * second_factor.eval(values)
}

pub fn div(dividend: &Expression, divisor: &Expression, values: &HashMap<char, f64>) -> f64 {
    dividend.eval(values) / divisor.eval(values)
}
