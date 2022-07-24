use std::collections::HashMap;

use crate::{
    function::{Function, FunctionType},
    value::Value,
    EvalResult,
};
use lazy_static::*;

lazy_static! {
    pub static ref CONSTANTS: HashMap<&'static str, Value> = {
        let mut m = HashMap::new();
        use std::f64::consts;
        m.insert("pi", Value::Float(consts::PI));
        m.insert("e", Value::Float(consts::E));
        m.insert("tau", Value::Float(consts::TAU));
        m.insert("phi", Value::Float(1.618_033_988_749_894));

        m
    };
    pub static ref BUILTIN_FUNCTIONS: Vec<Function> = vec![
        Function::new("sin", sin, FunctionType::Unary),
        Function::new("cos", cos, FunctionType::Unary),
        Function::new("tan", tan, FunctionType::Unary)
    ];
}

/// Returns `Some(Function)` if the identifier matches some.
pub fn built_in_function(identifier: &'static str) -> Option<Function> {
    BUILTIN_FUNCTIONS
        .iter()
        .find(|x| x.func_identifier == identifier)
        .cloned()
}

/// Returns `Some(Value)` if the identifier matches some.
pub fn consts(identifier: &'static str) -> Option<Value> {
    CONSTANTS.get(identifier).cloned()
}

// Built in functions:
fn sin(value: Value) -> EvalResult<Value> {
    let original_type = value.get_type();
    let value = value.as_complex()?;
    Ok(Value::Complex(value.sin()).try_as_type(original_type))
}

fn cos(value: Value) -> EvalResult<Value> {
    let original_type = value.get_type();
    let value = value.as_complex()?;
    Ok(Value::Complex(value.cos()).try_as_type(original_type))
}

fn tan(value: Value) -> EvalResult<Value> {
    let original_type = value.get_type();
    let value = value.as_complex()?;
    Ok(Value::Complex(value.tan()).try_as_type(original_type))
}
