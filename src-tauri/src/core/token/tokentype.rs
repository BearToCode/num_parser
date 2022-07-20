/// Contains all the possible input tokens type.
#[derive(PartialEq, Clone)]
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
