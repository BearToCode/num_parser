use super::ErrorType::{self, *};
use std::fmt;

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
            InvalidTokenPosition { token } => {
                write!(f, "SYNTAX ERROR: invalid position for token `{}`.", token)
            }
            FailedCast { value, from, to } => write!(
                f,
                "MATH ERROR: could not cast value `{}` from type {} to type {}.",
                value, from, to
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
            EmptyBrackets => write!(f, "SYNTAX ERROR: invalid empty brackets."),
            WrongFunctionArgumentsAmount {
                func_name,
                expected,
                given,
            } => write!(
                f,
                "SYNTAX ERROR: function `{}` expected {} arguments, but got {}.",
                func_name, expected, given
            ),
            MissingFunctionParameters { func_name } => write!(
                f,
                "SYNTAX ERROR: no arguments provided for function `{}`.",
                func_name
            ),
            InvalidDeclaration => write!(f, "SYNTAX ERROR: invalid declaration."),
            UnknownFunction { func_name } => {
                write!(f, "SYNTAX ERROR: unknown function `{}`.", func_name)
            }
            UnknownVar { var_name } => write!(f, "SYNTAX ERROR: unknown variable `{}`.", var_name),
            ReservedVarName { var_name } => write!(
                f,
                "INTERNAL ERROR: `{}` is a keyword and cannot be used as a variable name.",
                var_name
            ),
            ReservedFunctionName { func_name } => write!(
                f,
                "INTERNAL ERROR: `{}` is a keyword cannot be used as a function name.",
                func_name
            ),

            ErrorDuring {
                operation_name,
                error,
            } => write!(
                f,
                "An error occurred during operation `{}`: \n {}",
                operation_name, *error
            ),
            InternalError { message } => write!(f, "INTERNAL ERROR: {}.", message),
        }
    }
}
