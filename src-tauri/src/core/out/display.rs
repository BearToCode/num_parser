use super::ErrorType::{self, *};
use std::fmt::{self, write};

impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeError { expected, given } => write!(
                f,
                "MATH ERROR: a function expected type {}, but a type {} was given.",
                expected, given
            ),
            UnknownToken { token } => write!(
                f,
                "SYNTAX ERROR: an invalid token was provided: `{}`.",
                token
            ),
            InvalidTokenAtPosition { token } => {
                write!(f, "SYNTAX ERROR: invalid position for token `{}`.", token)
            }
            FailedCast { value, from, To } => write!(
                f,
                "MATH ERROR: could not cast value `{}` from type {} to type {}.",
                value, from, To
            ),
            MismatchedArrayLengths {
                first,
                second,
                operation_name,
            } => write!(
                f,
                "MATH ERROR: invalid vectors sizes {} and {} for operation `{}`.",
                first, second, operation_name
            ),
            DivideByZero { numerator } => {
                write!(f, "MATH ERROR: trying to divide {} by zero.", numerator)
            }
            NotAnOperator { token } => {
                write!(f, "SYNTAX ERROR: `{}` is not a valid operator!", token)
            }
            InvalidClosingBracket => write!(f, "SYNTAX ERROR: invalid closing bracket."),
            MissingClosingBracket => write!(f, "SYNTAX ERROR: missing closing bracket."),
            MissingOperatorArgument { token } => {
                write!(f, "SYNTAX ERROR: missing argument for operator `{}`", token)
            }
            FailedParse { value } => write!(f, "SYNTAX ERROR: could not parse value `{}`.", value),

            ErrorDuring {
                operation_name,
                error,
            } => write!(
                f,
                "An error occurred during operation `{}`: \n {}",
                operation_name, *error
            ),
        }
    }
}
