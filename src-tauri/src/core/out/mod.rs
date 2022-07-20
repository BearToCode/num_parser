mod display;

use super::value::valuetype::ValueType;

pub type EvalResult<T> = Result<T, ErrorType>;

#[derive(Debug)]
pub enum ErrorType {
    /// A mismatched type.
    TypeError {
        expected: ValueType,
        given: ValueType,
    },
    /// An unknown token found while parsing the string.
    UnknownToken { token: String },
    /// A known token placed in an invalid position.
    InvalidTokenAtPosition { token: String },
}

impl std::error::Error for ErrorType {}
