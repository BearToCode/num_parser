mod display;
pub mod valuetype;

use self::valuetype::ValueType;
use super::out::*;
use crate::token::tokentype::TokenType;
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
    pub fn get_type(&self) -> ValueType {
        match self {
            Value::Float(_) => ValueType::FloatType,
            Value::Int(_) => ValueType::IntType,
            Value::Complex(_) => ValueType::ComplexType,
            Value::Vector(_) => ValueType::VectorType,
            Value::Bool(_) => ValueType::BoolType,
        }
    }

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
            Value::Float(n) => {
                if n.fract() == 0.0 {
                    Ok(*n as IntValue)
                } else {
                    Err(ErrorType::FailedCast {
                        value: self.clone(),
                        from: ValueType::FloatType,
                        to: ValueType::IntType,
                    })
                }
            }
            Value::Bool(n) => Ok(*n as i64),
            _ => match self.as_float() {
                Ok(float) => Value::Float(float).as_int(),
                // Overwrite error with the current types
                Err(err) => match err {
                    ErrorType::FailedCast { value, from, to: _ } => Err(ErrorType::FailedCast {
                        value,
                        from,
                        to: ValueType::IntType,
                    }),
                    other => Err(other),
                },
            },
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
                        to: ValueType::FloatType,
                    })
                }
            }
            _ => match self.as_complex() {
                Ok(complex) => Value::Complex(complex).as_float(),
                // Overwrite error with the current types
                Err(err) => match err {
                    ErrorType::FailedCast { value, from, to: _ } => Err(ErrorType::FailedCast {
                        value,
                        from,
                        to: ValueType::FloatType,
                    }),
                    other => Err(other),
                },
            },
        }
    }

    pub fn as_complex(&self) -> EvalResult<ComplexValue> {
        match self {
            Value::Complex(n) => Ok(*n),
            Value::Float(n) => Ok(Complex64::new(*n, 0.0)),
            Value::Int(n) => Ok(Complex64::new(*n as f64, 0.0)),
            Value::Bool(n) => Ok(Complex64::new(*n as i64 as f64, 0.0)),
            Value::Vector(v) => {
                if v.len() == 1 {
                    v[1].as_complex()
                } else {
                    Err(ErrorType::FailedCast {
                        value: self.clone(),
                        from: ValueType::VectorType,
                        to: ValueType::ComplexType,
                    })
                }
            }
        }
    }

    pub fn as_vector(&self) -> VectorValue {
        match self {
            Value::Vector(v) => v.clone(),
            Value::Int(n) => vec![Value::Int(*n)],
            Value::Float(n) => vec![Value::Float(*n)],
            Value::Complex(n) => vec![Value::Complex(*n)],
            Value::Bool(n) => vec![Value::Bool(*n)],
        }
    }

    pub fn as_bool(&self) -> EvalResult<BoolValue> {
        match self {
            Value::Bool(n) => Ok(*n),
            Value::Int(n) => {
                if *n == 1 {
                    Ok(true)
                } else if *n == 0 {
                    Ok(false)
                } else {
                    Err(ErrorType::FailedCast {
                        value: self.clone(),
                        from: ValueType::IntType,
                        to: ValueType::BoolType,
                    })
                }
            }
            _ => match self.as_int() {
                Ok(value) => Value::Int(value).as_bool(),
                // Overwrite error with the current types
                Err(err) => match err {
                    ErrorType::FailedCast { value, from, to: _ } => Err(ErrorType::FailedCast {
                        value,
                        from,
                        to: ValueType::BoolType,
                    }),
                    other => Err(other),
                },
            },
        }
    }

    pub fn as_type(&self, valuetype: &ValueType) -> EvalResult<Value> {
        match valuetype {
            ValueType::BoolType => Ok(Value::Bool(self.as_bool()?)),
            ValueType::IntType => Ok(Value::Int(self.as_int()?)),
            ValueType::FloatType => Ok(Value::Float(self.as_float()?)),
            ValueType::ComplexType => Ok(Value::Complex(self.as_complex()?)),
            ValueType::VectorType => Ok(Value::Vector(self.as_vector())),
        }
    }

    /// Creates a new value from a string.
    pub fn from_string(string: String) -> EvalResult<Self> {
        match &string[..] {
            "true" => Ok(Value::Bool(true)),
            "false" => Ok(Value::Bool(false)),
            other => {
                let mut other = String::from(other);

                // Check for imaginary numbers
                let i_count = other.matches("i").count();
                if i_count != 0 {
                    if i_count == 1 {
                        other = String::from(other).replace("i", "");

                        let other = if other == "" {
                            String::from("1")
                        } else {
                            other
                        };

                        return Ok(Value::Complex(Complex64::new(
                            0.0,
                            match other.parse::<f64>() {
                                Ok(value) => value,
                                Err(_) => return Err(ErrorType::FailedParse { value: string }),
                            },
                        )));
                    } else {
                        return Err(ErrorType::FailedParse { value: string });
                    }
                }

                // Check for floats
                let count = other.matches(".").count();
                if count != 0 {
                    if count == 1 {
                        match other.parse::<f64>() {
                            Ok(value) => Ok(Value::Float(value)),
                            Err(_) => Err(ErrorType::FailedParse { value: string }),
                        }
                    } else {
                        Err(ErrorType::InvalidTokenAtPosition {
                            token: TokenType::Dot,
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

    /// Tries to convert the value to the requested value-type. If it fails, it returns the lowest
    /// complexity value achieved.
    pub fn try_as_type(&self, valuetype: ValueType) -> Value {
        // If target complexity is same or higher, return the value
        if ValueType::highest_complexity(vec![&self.get_type(), &valuetype]) == self.get_type() {
            self.clone()
        } else {
            match valuetype {
                ValueType::BoolType => match self.as_bool() {
                    Ok(value) => Value::Bool(value),
                    Err(_) => self.try_as_type(ValueType::IntType),
                },
                ValueType::IntType => match self.as_int() {
                    Ok(value) => Value::Int(value),
                    Err(_) => self.try_as_type(ValueType::FloatType),
                },
                ValueType::FloatType => match self.as_float() {
                    Ok(value) => Value::Float(value),
                    Err(_) => self.try_as_type(ValueType::ComplexType),
                },
                ValueType::ComplexType => match self.as_complex() {
                    Ok(value) => Value::Complex(value),
                    Err(_) => self.try_as_type(ValueType::VectorType),
                },
                ValueType::VectorType => Value::Vector(self.as_vector()),
            }
        }
    }
}
