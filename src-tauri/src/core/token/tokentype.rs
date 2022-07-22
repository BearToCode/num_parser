use self::TokenType::*;
use super::super::out::{ErrorType, EvalResult};

/// Contains all the possible input tokens type.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    // OPERATORS
    /// A plus '+' character.
    Plus,
    /// A minus '-' character.
    Minus,
    /// A star '*' character.
    Star,
    /// A slash '/' character.
    Slash,

    /// An opening bracket '(' character.
    OpeningBracket,
    /// A closing bracket ')' character.
    ClosingBracket,

    /// A dot '.' character.
    Comma,
    /// A string representing a value.
    Literal,
    /// A string representing either a function call or a variable.
    Identifier,
}

impl TokenType {
    pub fn is_expression(&self) -> bool {
        *self == TokenType::Identifier || // Function or variable
        *self == TokenType::Literal ||  // A number
        self.is_binary_operator() // An operator
    }

    pub fn is_binary_operator(&self) -> bool {
        match self {
            Plus | Minus | Star | Slash => true,
            _ => false,
        }
    }

    pub fn precedence(&self) -> EvalResult<u16> {
        Ok(match self {
            Plus | Minus => 10,
            Star | Slash => 20,

            Identifier => 200,
            Literal => 300,
            _ => return Err(ErrorType::NotAnOperator { token: *self }),
        })
    }

    pub fn max_precedence() -> u16 {
        500
    }
}
