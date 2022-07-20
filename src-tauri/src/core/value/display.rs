use super::valuetype::ValueType::{self, *};
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
