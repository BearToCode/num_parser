use super::{
    valuetype::ValueType::{self, *},
    Value,
};
use std::fmt;

impl fmt::Display for ValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IntType => write!(f, "Integer"),
            Self::FloatType => write!(f, "Float"),
            Self::ComplexType => write!(f, "Complex"),
            Self::VectorType => write!(f, "Vector"),
            Self::BoolType => write!(f, "Bool"),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bool(v) => write!(f, "{}", v),
            Self::Int(v) => write!(f, "{}", v),
            Self::Float(v) => write!(f, "{}", v),
            Self::Complex(v) => write!(f, "{}", v),
            Self::Vector(v) => write!(f, "{:?}", v),
        }
    }
}
