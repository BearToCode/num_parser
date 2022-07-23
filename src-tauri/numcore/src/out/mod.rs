mod display;

use super::{
    token::{tokentype::TokenType, Token},
    value::{valuetype::ValueType, Value},
};

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
    InvalidTokenAtPosition { token: TokenType },
    /// A failed cast due to data loss.
    FailedCast {
        value: Value,
        from: ValueType,
        to: ValueType,
    },
    /// Two arrays with different lengths.
    MismatchedArrayLengths {
        first: usize,
        second: usize,
        operation_name: &'static str,
    },
    /// Trying to divide by zero.
    DivideByZero { numerator: Value },
    /// A token which is not an operator being used as such.
    NotAnOperator { token: TokenType },
    /// An invalid closing bracket.
    InvalidClosingBracket,
    /// A missing closing bracket.
    MissingClosingBracket,
    /// A missing left argument for an operator.
    MissingOperatorArgument { token: TokenType },
    /// An error occurred while parsing a literal.
    FailedParse { value: String },
    /// Two brackets with nothing inside.
    EmptyBrackets,

    /// An error wrapper to add additional information.
    ErrorDuring {
        operation_name: &'static str,
        error: Box<ErrorType>,
    },
}

impl std::error::Error for ErrorType {}
