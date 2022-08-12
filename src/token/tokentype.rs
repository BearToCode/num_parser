use self::TokenType::*;
use super::super::out::{ErrorType, EvalResult};

#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// The expression represented by the identifier.
pub enum IdentifierType {
    Var,
    Function,
    Unknown,
}

/// Contains all the possible input tokens type.
#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// All possible tokens to be interpreted.
pub enum TokenType {
    // Operators
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
    /// A caret '^' character.
    Caret,
    /// A percentage '%' character.
    Percentage,
    /// A less-than sign '<' character.
    LessThan,
    /// A greater-than sign '>' character.
    GreaterThan,
    /// An and '&' character.
    And,
    /// An or '|' character.
    Or,

    // TO BE ASSEMBLED
    /// A less-than sign followed by an equal character.
    LessOrEqualTo,
    /// A greater-than sign followed by an equal character.
    GreaterOrEqualTo,
    /// A double equal '=' characters.
    DoubleEqual,
    /// Two and '&' characters.
    DoubleAnd,
    /// Two or '|' characters.
    DoubleOr,
    /// An exclamation point followed by an equal character.
    NotEqual,

    /// An exclamation point '!' character.
    Exclamation,

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
        self.is_binary_operator() || // An operator
        self.is_unary_operator() ||
        self.is_union_operator()
    }

    pub fn is_binary_operator(&self) -> bool {
        match self {
            Plus | Minus | Star | Slash | Equal | Caret | Percentage | LessThan | GreaterThan
            | LessOrEqualTo | GreaterOrEqualTo | DoubleAnd | DoubleOr | DoubleEqual | NotEqual => {
                true
            }
            _ => false,
        }
    }

    pub fn is_unary_operator(&self) -> bool {
        match self {
            Minus | Exclamation => true,
            _ => false,
        }
    }

    pub fn is_union_operator(&self) -> bool {
        match self {
            Comma => true,
            _ => false,
        }
    }

    pub fn precedence(&self) -> EvalResult<u16> {
        Ok(match self {
            Literal => 300,
            Identifier(_) => 200,
            Caret => 90,
            Exclamation => 80,
            Star | Slash | Percentage => 70,
            Plus | Minus => 60,
            LessThan | LessOrEqualTo | GreaterThan | GreaterOrEqualTo => 50,
            DoubleEqual | NotEqual => 40,
            DoubleAnd => 30,
            DoubleOr => 20,
            Comma => 10,
            Equal => 0,

            _ => return Err(ErrorType::NotAnOperator { token: *self }),
        })
    }
}
