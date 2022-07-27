mod display;

use super::{
    token::tokentype::TokenType,
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
    InvalidTokenPosition { token: TokenType },
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
    /// A function call with the wrong function arguments amount.
    WrongFunctionArgumentsAmount {
        func_name: String,
        expected: u8,
        given: u8,
    },
    /// A function with no parameters.
    MissingFunctionParameters { func_name: String },
    /// An invalid declaration.
    InvalidDeclaration,
    /// An unknown function.
    UnknownFunction { func_name: String },
    /// An unknown variable.
    UnknownVar { var_name: String },
    /// A reserved variable name.
    ReservedVarName { var_name: String },
    /// A reserved function name.
    ReservedFunctionName { func_name: String },

    /// An error wrapper to add additional information.
    ErrorDuring {
        operation_name: &'static str,
        error: Box<ErrorType>,
    },
    /// An error due to a missing implementation or a bug. This should
    /// never occur.
    InternalError { message: String },
}

impl std::error::Error for ErrorType {}
