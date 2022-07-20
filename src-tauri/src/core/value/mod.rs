mod display;
pub mod valuetype;

use self::valuetype::ValueType::*;
use super::out::EvalResult;
use super::out::*;
use num::complex::Complex64;

pub type IntValue = i64;
pub type FloatValue = f64;
pub type ComplexValue = Complex64;
pub type VectorValue = Vec<Value>;
pub type BoolValue = bool;

/// Every possible value.
#[derive(Debug, Clone)]
pub enum Value {
    Int(IntValue),
    Float(FloatValue),
    Complex(ComplexValue),
    Vector(VectorValue),
    Bool(BoolValue),
}

impl Value {
    pub fn as_int(&self) -> EvalResult<IntValue> {
        match self {
            Value::Int(n) => Ok(*n),
            Value::Bool(n) => Ok(*n as i64),
            other => Err(ErrorType::TypeError {
                expected: IntType,
                given: other.to_type(),
            }),
        }
    }

    pub fn as_float(&self) -> EvalResult<FloatValue> {
        match self {
            Value::Float(n) => Ok(*n),
            Value::Int(n) => Ok(*n as f64),
            Value::Bool(n) => Ok(*n as i64 as f64),
            other => Err(ErrorType::TypeError {
                expected: FloatType,
                given: other.to_type(),
            }),
        }
    }

    pub fn as_complex(&self) -> EvalResult<ComplexValue> {
        match self {
            Value::Complex(n) => Ok(*n),
            Value::Float(n) => Ok(Complex64::new(*n, 0.0)),
            Value::Int(n) => Ok(Complex64::new(*n as f64, 0.0)),
            Value::Bool(n) => Ok(Complex64::new(*n as i64 as f64, 0.0)),
            other => Err(ErrorType::TypeError {
                expected: ComplexType,
                given: other.to_type(),
            }),
        }
    }

    pub fn as_vector(&self) -> EvalResult<VectorValue> {
        match self {
            Value::Vector(v) => Ok(v.clone()),
            Value::Int(n) => Ok(vec![Value::Int(*n)]),
            Value::Float(n) => Ok(vec![Value::Float(*n)]),
            Value::Complex(n) => Ok(vec![Value::Complex(*n)]),
            Value::Bool(n) => Ok(vec![Value::Bool(*n)]),
        }
    }

    pub fn as_bool(&self) -> EvalResult<BoolValue> {
        match self {
            Value::Bool(n) => Ok(*n),
            other => Err(ErrorType::TypeError {
                expected: BoolType,
                given: other.to_type(),
            }),
        }
    }
}
