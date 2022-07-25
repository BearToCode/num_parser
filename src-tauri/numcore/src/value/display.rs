use super::{valuetype::ValueType, Value};
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
            Self::Complex(v) => write!(
                f,
                "{}",
                if v.re == 0.0 {
                    if v.im == 1.0 {
                        String::from("i")
                    } else {
                        format!("{}i", v.im)
                    }
                } else if v.im == 0.0 {
                    format!("{}", v.re)
                } else {
                    format!(
                        "{}{}{}i",
                        v.re,
                        if v.im > 0.0 { "+" } else { "-" },
                        if v.im.abs() == 1.0 {
                            String::from("")
                        } else {
                            format!("{}", v.im.abs())
                        }
                    )
                }
            ),
            Self::Vector(v) => write!(f, "{:?}", v),
        }
    }
}
