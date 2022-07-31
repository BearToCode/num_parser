use std::collections::HashMap;

use crate::{
    function::{Arguments, Function},
    out::ErrorType,
    value::Value,
    EvalResult, ValueType,
};
use lazy_static::*;
use rand::Rng;
use tuple_conv::RepeatedTuple;

lazy_static! {
    pub static ref CONSTANTS: HashMap<&'static str, Value> = {
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
    };
    pub static ref BUILT_IN_FUNCTIONS: Vec<Function> = vec![
        Function::new("min", min, Arguments::Dynamic),
        Function::new("max", max, Arguments::Dynamic),
        Function::new("floor", floor, Arguments::Const(1)),
        Function::new("ceil", ceil, Arguments::Const(1)),
        Function::new("round", round, Arguments::Const(1)),
        Function::new("ln", ln, Arguments::Const(1)),
        Function::new("log", log, Arguments::Const(2)),
        Function::new("exp", exp, Arguments::Const(1)),
        Function::new("rand", rand, Arguments::Const(2)),

        Function::new("branch", branch, Arguments::Const(3)),

        Function::new("sin", sin, Arguments::Const(1)),
        Function::new("cos", cos, Arguments::Const(1)),
        Function::new("tan", tan, Arguments::Const(1)),
        Function::new("asin", asin, Arguments::Const(1)),
        Function::new("acos", acos, Arguments::Const(1)),
        Function::new("atan", atan, Arguments::Const(1)),
        Function::new("sinh", sinh, Arguments::Const(1)),
        Function::new("cosh", cosh, Arguments::Const(1)),
        Function::new("tanh", tanh, Arguments::Const(1)),
        Function::new("asinh", asinh, Arguments::Const(1)),
        Function::new("acosh", acosh, Arguments::Const(1)),
        Function::new("atanh", atanh, Arguments::Const(1)),

        Function::new("re", re, Arguments::Const(1)),
        Function::new("im", im, Arguments::Const(1)),
        Function::new("polar", polar, Arguments::Const(1)),
        Function::new("arg", arg, Arguments::Const(1)),
        Function::new("norm", norm, Arguments::Const(1)),

    ];
}

/// Returns `Some(Function)` if the identifier matches some.
pub fn get_function(identifier: &str) -> Option<Function> {
    BUILT_IN_FUNCTIONS
        .iter()
        .find(|x| x.func_identifier == identifier)
        .cloned()
}

/// Returns `Some(Value)` if the identifier matches some.
pub fn get_const(identifier: &str) -> Option<Value> {
    CONSTANTS.get(identifier).cloned()
}

/// Returns all reserved keywords.
pub fn reserved_keywords() -> Vec<&'static str> {
    [
        CONSTANTS.keys().cloned().collect::<Vec<&'static str>>(),
        BUILT_IN_FUNCTIONS
            .iter()
            .map(|x| x.func_identifier)
            .collect(),
    ]
    .concat()
}

fn fn_wrapper<P, T>(value: Value, target_type: ValueType, mut predicate: P) -> EvalResult<Value>
where
    P: FnMut(Value) -> EvalResult<T>,
    Value: From<T>,
{
    let original_type = value.get_type();
    let value = value.as_type(&target_type)?;
    Ok(Value::from(predicate(value)?).try_as_type(original_type))
}

macro_rules! read_vec_values {
    ( $vec:expr, $($x:ident),* ) => {
        let vec = $vec.as_vector();
        let mut iter = vec.iter();

        $(
            let $x = match iter.next() {
                Some(value) => value,
                None => return Err(ErrorType::InternalError { message: "failed to retrieve function parameters".to_owned() })
            };
        )*
    };
}

// STD

fn min(value: Value) -> EvalResult<Value> {
    fn_wrapper(value, ValueType::VectorType, |v| {
        let vec = v.as_vector();
        let mut min = vec[0].as_float()?;
        for elem in vec {
            if elem.as_float()? < min {
                min = elem.as_float()?;
            }
        }
        Ok(Value::Float(min))
    })
}

fn max(value: Value) -> EvalResult<Value> {
    fn_wrapper(value, ValueType::VectorType, |v| {
        let vec = v.as_vector();
        let mut min = vec[0].as_float()?;
        for elem in vec {
            if elem.as_float()? > min {
                min = elem.as_float()?;
            }
        }
        Ok(Value::Float(min))
    })
}

fn floor(value: Value) -> EvalResult<Value> {
    fn_wrapper(value, ValueType::FloatType, |v| Ok(v.as_float()?.floor()))
}

fn ceil(value: Value) -> EvalResult<Value> {
    fn_wrapper(value, ValueType::FloatType, |v| Ok(v.as_float()?.ceil()))
}

fn round(value: Value) -> EvalResult<Value> {
    fn_wrapper(value, ValueType::FloatType, |v| Ok(v.as_float()?.round()))
}

fn ln(value: Value) -> EvalResult<Value> {
    fn_wrapper(value, ValueType::ComplexType, |v| Ok(v.as_float()?.ln()))
}

fn log(value: Value) -> EvalResult<Value> {
    fn_wrapper(value, ValueType::VectorType, |v| {
        read_vec_values!(v, base, argument);
        Ok(argument.as_complex()?.log(base.as_float()?))
    })
}

fn exp(value: Value) -> EvalResult<Value> {
    fn_wrapper(value, ValueType::ComplexType, |v| {
        Value::exponentiation(CONSTANTS.get("e").unwrap().clone(), v)
    })
}

fn rand(value: Value) -> EvalResult<Value> {
    fn_wrapper(value, ValueType::VectorType, |v| {
        read_vec_values!(v, min, max);
        Ok(Value::Float(
            rand::thread_rng().gen_range(min.as_float()?..max.as_float()?),
        ))
    })
}

// LOGIC

fn branch(value: Value) -> EvalResult<Value> {
    fn_wrapper(value, ValueType::VectorType, |v| {
        read_vec_values!(v, condition, a, b);
        if condition.as_bool()? {
            Ok(a.clone())
        } else {
            Ok(b.clone())
        }
    })
}

// TRIGONOMETRY

fn sin(value: Value) -> EvalResult<Value> {
    fn_wrapper(value, ValueType::ComplexType, |v| Ok(v.as_complex()?.sin()))
}

fn cos(value: Value) -> EvalResult<Value> {
    fn_wrapper(value, ValueType::ComplexType, |v| Ok(v.as_complex()?.cos()))
}

fn tan(value: Value) -> EvalResult<Value> {
    fn_wrapper(value, ValueType::ComplexType, |v| Ok(v.as_complex()?.tan()))
}

fn asin(value: Value) -> EvalResult<Value> {
    fn_wrapper(value, ValueType::ComplexType, |v| {
        Ok(v.as_complex()?.asin())
    })
}

fn acos(value: Value) -> EvalResult<Value> {
    fn_wrapper(value, ValueType::ComplexType, |v| {
        Ok(v.as_complex()?.acos())
    })
}

fn atan(value: Value) -> EvalResult<Value> {
    fn_wrapper(value, ValueType::ComplexType, |v| {
        Ok(v.as_complex()?.atan())
    })
}

fn sinh(value: Value) -> EvalResult<Value> {
    fn_wrapper(value, ValueType::ComplexType, |v| {
        Ok(v.as_complex()?.sinh())
    })
}

fn cosh(value: Value) -> EvalResult<Value> {
    fn_wrapper(value, ValueType::ComplexType, |v| {
        Ok(v.as_complex()?.cosh())
    })
}

fn tanh(value: Value) -> EvalResult<Value> {
    fn_wrapper(value, ValueType::ComplexType, |v| {
        Ok(v.as_complex()?.tanh())
    })
}

fn asinh(value: Value) -> EvalResult<Value> {
    fn_wrapper(value, ValueType::ComplexType, |v| {
        Ok(v.as_complex()?.asinh())
    })
}

fn acosh(value: Value) -> EvalResult<Value> {
    fn_wrapper(value, ValueType::ComplexType, |v| {
        Ok(v.as_complex()?.acosh())
    })
}

fn atanh(value: Value) -> EvalResult<Value> {
    fn_wrapper(value, ValueType::ComplexType, |v| {
        Ok(v.as_complex()?.atanh())
    })
}

// COMPLEX

fn re(value: Value) -> EvalResult<Value> {
    fn_wrapper(value, ValueType::ComplexType, |v| Ok(v.as_complex()?.re))
}

fn im(value: Value) -> EvalResult<Value> {
    fn_wrapper(value, ValueType::ComplexType, |v| Ok(v.as_complex()?.im))
}

fn polar(value: Value) -> EvalResult<Value> {
    fn_wrapper(value, ValueType::ComplexType, |v| {
        Ok(v.as_complex()?.to_polar().to_vec())
    })
}

fn arg(value: Value) -> EvalResult<Value> {
    fn_wrapper(value, ValueType::ComplexType, |v| Ok(v.as_complex()?.arg()))
}

fn norm(value: Value) -> EvalResult<Value> {
    fn_wrapper(value, ValueType::ComplexType, |v| {
        Ok(v.as_complex()?.norm())
    })
}
