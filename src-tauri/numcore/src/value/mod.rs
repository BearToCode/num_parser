mod display;
pub mod valuetype;

use self::valuetype::ValueType::{self, *};
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
    pub fn is_int(&self) -> bool {
        matches!(self, Value::Int(_))
    }

    pub fn is_float(&self) -> bool {
        matches!(self, Value::Float(_))
    }

    pub fn is_complex(&self) -> bool {
        matches!(self, Value::Complex(_))
    }

    pub fn is_vector(&self) -> bool {
        matches!(self, Value::Vector(_))
    }

    pub fn is_bool(&self) -> bool {
        matches!(self, Value::Bool(_))
    }

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
            Value::Complex(n) => {
                if n.im == 0.0 {
                    Ok(n.re)
                } else {
                    Err(ErrorType::FailedCast {
                        value: self.clone(),
                        from: ValueType::ComplexType,
                        To: ValueType::FloatType,
                    })
                }
            }
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

    pub fn as_type(&self, valuetype: &ValueType) -> EvalResult<Value> {
        match valuetype {
            ValueType::BoolType => Ok(Value::Bool(self.as_bool()?)),
            ValueType::IntType => Ok(Value::Int(self.as_int()?)),
            ValueType::FloatType => Ok(Value::Float(self.as_float()?)),
            ValueType::ComplexType => Ok(Value::Complex(self.as_complex()?)),
            ValueType::VectorType => Ok(Value::Vector(self.as_vector()?)),
        }
    }

    /// Creates a new value from a string. Only accepts booleans, ints or floats.
    pub fn from_string(string: String) -> EvalResult<Self> {
        match &string[..] {
            "true" => Ok(Value::Bool(true)),
            "false" => Ok(Value::Bool(false)),
            other => {
                let count = other.matches(".").count();
                if count != 0 {
                    if count == 1 {
                        match other.parse::<f64>() {
                            Ok(value) => Ok(Value::Float(value)),
                            Err(_) => Err(ErrorType::FailedParse { value: string }),
                        }
                    } else {
                        Err(ErrorType::InvalidTokenAtPosition {
                            token: super::token::tokentype::TokenType::Dot,
                        })
                    }
                } else {
                    match other.parse::<i64>() {
                        Ok(value) => Ok(Value::Int(value)),
                        Err(_) => Err(ErrorType::FailedParse { value: string }),
                    }
                }
            }
        }
    }
}
