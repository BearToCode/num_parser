use crate::value::Value;

/// Contains all possible values types.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ValueType {
    /// Type for `i64`
    IntType,
    /// Type for `f64`
    FloatType,
    /// Type for `num::complex::Complex`
    ComplexType,
    /// Type for `Vec<Value>`
    VectorType,
    /// Type for `bool`
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
    /// Returns a `u8` higher the more complex the valuetype is. Useful for
    /// comparisons.
    pub fn complexity(&self) -> u8 {
        match self {
            Self::BoolType => 1,
            Self::IntType => 2,
            Self::FloatType => 3,
            Self::ComplexType => 4,
            Self::VectorType => 5,
        }
    }

    /// Returns the highest complexity value type of all one provided.
    pub fn highest_complexity(types: Vec<&ValueType>) -> ValueType {
        let mut highest = ValueType::BoolType;
        for t in types {
            if t.complexity() > highest.complexity() {
                highest = t.clone();
            }
        }
        highest
    }

    /// Returns the lowest complexity value type of all the one provided.
    pub fn lowest_complexity(types: Vec<&ValueType>) -> ValueType {
        let mut lowest = ValueType::VectorType;
        for t in types {
            if t.complexity() < lowest.complexity() {
                lowest = t.clone();
            }
        }
        lowest
    }
}
