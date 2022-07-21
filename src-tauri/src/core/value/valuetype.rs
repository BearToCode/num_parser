use std::vec;

use super::Value;

#[derive(Debug, PartialEq, Clone)]
pub enum ValueType {
    IntType,
    FloatType,
    ComplexType,
    VectorType,
    BoolType,
}

impl Value {
    pub fn to_type(&self) -> ValueType {
        match self {
            Self::Int(_) => ValueType::IntType,
            Self::Float(_) => ValueType::FloatType,
            Self::Complex(_) => ValueType::ComplexType,
            Self::Vector(_) => ValueType::VectorType,
            Self::Bool(_) => ValueType::BoolType,
        }
    }
}

impl ValueType {
    fn complexity(&self) -> u8 {
        match self {
            Self::BoolType => 1,
            Self::IntType => 2,
            Self::FloatType => 3,
            Self::ComplexType => 4,
            Self::VectorType => 5,
        }
    }

    pub fn highest_complexity(types: Vec<ValueType>) -> ValueType {
        let mut highest = ValueType::BoolType;
        for t in types {
            if t.complexity() > highest.complexity() {
                highest = t;
            }
        }
        highest
    }
}
