pub mod valuetype;

use self::valuetype::ValueType::*;
use super::out::EvalResult::*;
use super::out::*;

pub type IntValue = i64;
pub type FloatValue = f64;
pub type ComplexValue = (f64, f64);
pub type VectorValue = Vec<Value>;

/// Every possible value.
pub enum Value {
    Int(IntValue),
    Float(FloatValue),
    Complex(ComplexValue),
    Vector(VectorValue),
}

impl Value {
    pub fn as_int(&self) -> EvalResult<IntValue> {
        match self {
            Value::Int(n) => Ok(*n),
            other => EvalError(ErrorType::TypeError {
                expected: other.to_type(),
                given: IntType,
            }),
        }
    }
}
