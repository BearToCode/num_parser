//!
//! Contains functions to easily retrieve and set built-in functions and constants.
//!

use std::collections::HashMap;

use crate::{
    create_func, decl_func, function::Function, function::*, out::ErrorType, read_vec_values,
    value::Value, EvalResult, ValueType,
};
use lazy_static::*;
use num::complex::ComplexFloat;
use rand::Rng;
use std::sync::RwLock;
use tuple_conv::RepeatedTuple;

lazy_static! {
    #[derive(Debug, Clone)]
    static ref CONSTANTS: RwLock<HashMap<&'static str, Value>> = RwLock::new
    ({
        let mut m = HashMap::new();
        use std::f64::consts;
        // Math constants
        m.insert("pi", Value::Float(consts::PI));
        m.insert("e", Value::Float(consts::E));
        m.insert("tau", Value::Float(consts::TAU));
        m.insert("phi", Value::Float(1.618_033_988_749_894));

        // Literal values
        m.insert("true", Value::Bool(true));
        m.insert("false", Value::Bool(false));
        m.insert("i", Value::Complex(num::Complex::i()));

        m
    });
    #[derive(Debug, Clone)]
    static ref BUILT_IN_FUNCTIONS: RwLock<Vec<Function>> = RwLock::new(vec![
        create_func!(min, Arguments::Dynamic),
        create_func!(max, Arguments::Dynamic),
        create_func!(floor, Arguments::Const(1)),
        create_func!(ceil, Arguments::Const(1)),
        create_func!(round, Arguments::Const(1)),
        create_func!(abs, Arguments::Const(1)),
        create_func!(sqrt, Arguments::Const(1)),
        create_func!(ln, Arguments::Const(1)),
        create_func!(log, Arguments::Const(2)),
        create_func!(exp, Arguments::Const(1)),
        create_func!(rand, Arguments::Const(2)),

        create_func!(branch, Arguments::Const(3)),

        create_func!(sin, Arguments::Const(1)),
        create_func!(cos, Arguments::Const(1)),
        create_func!(tan, Arguments::Const(1)),
        create_func!(asin, Arguments::Const(1)),
        create_func!(acos, Arguments::Const(1)),
        create_func!(atan, Arguments::Const(1)),
        create_func!(sinh, Arguments::Const(1)),
        create_func!(cosh, Arguments::Const(1)),
        create_func!(tanh, Arguments::Const(1)),
        create_func!(asinh, Arguments::Const(1)),
        create_func!(acosh, Arguments::Const(1)),
        create_func!(atanh, Arguments::Const(1)),

        create_func!(re, Arguments::Const(1)),
        create_func!(im, Arguments::Const(1)),
        create_func!(polar, Arguments::Const(1)),
        create_func!(arg, Arguments::Const(1)),
        create_func!(norm, Arguments::Const(1)),

    ]);
}

/// Returns `Some(Function)` if the identifier matches some.
pub fn get_built_in_function(identifier: &str) -> Option<Function> {
    get_built_in_functions_vec()
        .iter()
        .find(|x| x.func_identifier == identifier)
        .cloned()
}

/// Returns `Some(Value)` if the identifier matches some.
pub fn get_built_in_const(identifier: &str) -> Option<Value> {
    get_built_in_consts_map()
        .iter()
        .find(|&x| x.0 == identifier)
        .map(|x| x.1.clone())
}

/// Returns all reserved keywords.
pub fn reserved_keywords<'a>() -> Vec<&'a str> {
    [
        get_built_in_consts_map()
            .iter()
            .map(|x| x.0)
            .collect::<Vec<&str>>(),
        get_built_in_functions_vec()
            .iter()
            .map(|x| x.func_identifier)
            .collect::<Vec<&str>>(),
    ]
    .concat()
}

/// Get a cloned vector of all built-in functions.
pub fn get_built_in_functions_vec() -> Vec<Function> {
    BUILT_IN_FUNCTIONS.read().unwrap().iter().cloned().collect()
}

/// Get a cloned vector of all built-in constants.
pub fn get_built_in_consts_map() -> Vec<(&'static str, Value)> {
    CONSTANTS
        .read()
        .unwrap()
        .iter()
        .map(|x| (x.0.clone(), x.1.clone()))
        .collect()
}

/// Add a function to the built-in ones.
pub fn add_built_in_function(func: Function) {
    BUILT_IN_FUNCTIONS.write().unwrap().push(func)
}

/// Add a constant to the built-in ones.
///
/// If a constant with the same identifier didn't exist, `None` is returned.
///
/// If it existed, the value is updated and the old value is returned.
pub fn add_built_in_const(identifier: &'static str, value: Value) -> Option<Value> {
    CONSTANTS.write().unwrap().insert(identifier, value)
}

/// Removes a built-in function with a matching identifier.
///
/// If a function is found, it is removed and returned, otherwise `None` is returned.
pub fn remove_built_in_function(func_identifier: &str) -> Option<Function> {
    if let Some(index) = BUILT_IN_FUNCTIONS
        .read()
        .unwrap()
        .iter()
        .position(|x| x.func_identifier == func_identifier)
    {
        Some(BUILT_IN_FUNCTIONS.write().unwrap().swap_remove(index))
    } else {
        None
    }
}

/// Removes a built-in constant with a matching identifier.
///
/// If a constant is found, it is removed and returned, otherwise `None` is returned.
pub fn remove_built_in_const(const_identifier: &str) -> Option<Value> {
    CONSTANTS.write().unwrap().remove(const_identifier)
}

// STD

decl_func!(
    min,
    FunctionType::Std,
    |v| {
        let vec = v.as_vector();
        let mut min = vec[0].as_float()?;
        for elem in vec {
            if elem.as_float()? < min {
                min = elem.as_float()?;
            }
        }
        Ok(Value::Float(min))
    },
    ValueType::VectorType
);

decl_func!(
    max,
    FunctionType::Std,
    |v| {
        let vec = v.as_vector();
        let mut max = vec[0].as_float()?;
        for elem in vec {
            if elem.as_float()? > max {
                max = elem.as_float()?;
            }
        }
        Ok(Value::Float(max))
    },
    ValueType::VectorType
);

decl_func!(
    floor,
    FunctionType::Std,
    |v| Ok(v.as_float()?.floor()),
    ValueType::FloatType
);

decl_func!(
    ceil,
    FunctionType::Std,
    |v| Ok(v.as_float()?.ceil()),
    ValueType::FloatType
);

decl_func!(
    round,
    FunctionType::Std,
    |v| Ok(v.as_float()?.round()),
    ValueType::FloatType
);

decl_func!(
    abs,
    FunctionType::Std,
    |v: Value| Ok(Value::Float(v.as_complex()?.abs())),
    ValueType::ComplexType
);

decl_func!(
    sqrt,
    FunctionType::Std,
    |v: Value| Ok(Value::Complex(v.as_complex()?.sqrt())),
    ValueType::ComplexType
);

decl_func!(
    ln,
    FunctionType::Std,
    |v| Ok(v.as_complex()?.ln()),
    ValueType::ComplexType
);

decl_func!(
    log,
    FunctionType::Std,
    |v| {
        read_vec_values!(v, base, argument);
        Ok(argument.as_complex()?.log(base.as_float()?))
    },
    ValueType::VectorType
);

decl_func!(
    exp,
    FunctionType::Std,
    |v| Ok(v.as_complex()?.exp()),
    ValueType::ComplexType
);

decl_func!(
    rand,
    FunctionType::Std,
    |v| {
        read_vec_values!(v, min, max);
        Ok(Value::Float(
            rand::thread_rng().gen_range(min.as_float()?..max.as_float()?),
        ))
    },
    ValueType::VectorType
);

// LOGIC

fn branch(arguments: &Vec<Box<Expression>>, context: &Context) -> EvalResult<Value> {
    let condition = arguments[0].eval(context, None)?.as_bool()?;
    if condition {
        Ok(arguments[1].eval(context, None)?)
    } else {
        Ok(arguments[2].eval(context, None)?)
    }
}

// TRIGONOMETRY

decl_func!(
    sin,
    FunctionType::Trig,
    |v| Ok(v.as_complex()?.sin()),
    ValueType::ComplexType
);

decl_func!(
    cos,
    FunctionType::Trig,
    |v| Ok(v.as_complex()?.cos()),
    ValueType::ComplexType
);

decl_func!(
    tan,
    FunctionType::Trig,
    |v| Ok(v.as_complex()?.tan()),
    ValueType::ComplexType
);

decl_func!(
    asin,
    FunctionType::InverseTrig,
    |v| Ok(v.as_complex()?.asin()),
    ValueType::ComplexType
);

decl_func!(
    acos,
    FunctionType::InverseTrig,
    |v| Ok(v.as_complex()?.acos()),
    ValueType::ComplexType
);

decl_func!(
    atan,
    FunctionType::InverseTrig,
    |v| Ok(v.as_complex()?.atan()),
    ValueType::ComplexType
);

decl_func!(
    sinh,
    FunctionType::Trig,
    |v| Ok(v.as_complex()?.sinh()),
    ValueType::ComplexType
);

decl_func!(
    cosh,
    FunctionType::Trig,
    |v| Ok(v.as_complex()?.cosh()),
    ValueType::ComplexType
);

decl_func!(
    tanh,
    FunctionType::Trig,
    |v| Ok(v.as_complex()?.tanh()),
    ValueType::ComplexType
);

decl_func!(
    asinh,
    FunctionType::InverseTrig,
    |v| Ok(v.as_complex()?.asinh()),
    ValueType::ComplexType
);

decl_func!(
    acosh,
    FunctionType::InverseTrig,
    |v| Ok(v.as_complex()?.acosh()),
    ValueType::ComplexType
);

decl_func!(
    atanh,
    FunctionType::InverseTrig,
    |v| Ok(v.as_complex()?.atanh()),
    ValueType::ComplexType
);

// COMPLEX

decl_func!(
    re,
    FunctionType::Std,
    |v| Ok(v.as_complex()?.re),
    ValueType::ComplexType
);

decl_func!(
    im,
    FunctionType::Std,
    |v| Ok(v.as_complex()?.im),
    ValueType::ComplexType
);

decl_func!(
    polar,
    FunctionType::Std,
    |v| Ok(v.as_complex()?.to_polar().to_vec()),
    ValueType::ComplexType
);

decl_func!(
    arg,
    FunctionType::Std,
    |v| Ok(v.as_complex()?.arg()),
    ValueType::ComplexType
);

decl_func!(
    norm,
    FunctionType::Std,
    |v| Ok(v.as_complex()?.norm()),
    ValueType::ComplexType
);
