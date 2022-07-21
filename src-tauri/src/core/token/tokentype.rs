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

    /// A dot '.' character.
    Comma,
    /// A string representing a value.
    Literal,
    /// A string representing either a function call or a variable.
    Identifier,
}

impl TokenType {
    pub fn is_binary_operator(&self) -> bool {
        use self::TokenType::*;
        match self {
            Plus | Minus | Star | Slash => true,
            _ => false,
        }
    }
}
