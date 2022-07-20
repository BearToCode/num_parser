use super::Value;

#[derive(Debug)]
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
