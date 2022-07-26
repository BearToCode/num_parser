use std::fmt;

use super::{
    tokentype::TokenType::{self, *},
    Token,
};

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.r#type {
            Plus | Minus | Star | Slash | Dot | Comma | OpeningBracket | ClosingBracket | Equal => {
                write!(f, "{}", self.r#type)
            }

            Literal | Identifier(_) => write!(f, "{}", self.value),
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
            Equal => writeln!(f, "="),
            Comma => write!(f, ","),

            OpeningBracket => write!(f, "("),
            ClosingBracket => write!(f, ")"),

            Dot => write!(f, "."),
            Literal => write!(f, "<literal>"),
            Identifier(i_type) => write!(f, "<identifier: {:?}>", i_type),
        }
    }
}
