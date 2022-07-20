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
            InvalidTokenAtPosition { token } => {
                write!(f, "SYNTAX ERROR: invalid position for token `{}`.", token)
            }
        }
    }
}
