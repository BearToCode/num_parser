use self::TokenType::*;
use super::super::out::{ErrorType, EvalResult};

#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum IdentifierType {
    Var,
    Function,
    Unknown,
}

/// Contains all the possible input tokens type.
#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    /// A comma ',' character.
    Comma,
    /// An equal '=' character.
    Equal,

    /// An opening bracket '(' character.
    OpeningBracket,
    /// A closing bracket ')' character.
    ClosingBracket,

    /// A dot '.' character.
    Dot,
    /// A string representing a value.
    Literal,

    /// A string representing a function, a constant or a variable.
    Identifier(IdentifierType),
}

impl TokenType {
    pub fn is_expression(&self) -> bool {
        matches!(*self, TokenType::Identifier(_)) || // An identifier
        *self == TokenType::Literal ||  // A number
        self.is_binary_operator() // An operator
    }

    pub fn is_binary_operator(&self) -> bool {
        match self {
            Plus | Minus | Star | Slash | Comma | Equal => true,
            _ => false,
        }
    }

    pub fn is_unary_operator(&self) -> bool {
        match self {
            Minus => true,
            _ => false,
        }
    }

    pub fn precedence(&self) -> EvalResult<u16> {
        Ok(match self {
            Equal => 10,
            Comma => 20,
            Plus | Minus => 30,
            Star | Slash => 40,

            Identifier(_) => 200,

            Literal => 300,
            _ => return Err(ErrorType::NotAnOperator { token: *self }),
        })
    }
}
