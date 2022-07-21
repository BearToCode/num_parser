use super::{
    tokentype::TokenType::{self, *},
    Token,
};
use std::fmt;

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.r#type {
            Plus | Minus | Star | Slash | Comma => write!(f, "{}", self.r#type),

            Literal => write!(f, "{}", self.value),
            Identifier => write!(f, "{}", self.value),
        }
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Plus => write!(f, "+"),
            Minus => write!(f, "-"),
            Star => write!(f, "*"),
            Slash => write!(f, "/"),

            Comma => write!(f, "."),
            Literal => write!(f, "<literal>"),
            Identifier => write!(f, "<identifier>"),
        }
    }
}
