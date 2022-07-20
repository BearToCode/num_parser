use super::Value;

pub enum ValueType {
    IntType,
    FloatType,
    ComplexType,
    VectorType,
}

impl Value {
    pub fn to_type(&self) -> ValueType {
        match self {
            Self::Int(_) => ValueType::IntType,
            Self::Float(_) => ValueType::FloatType,
            Self::Complex(_) => ValueType::ComplexType,
            Self::Vector(_) => ValueType::VectorType,
        }
    }
}
