use std::fmt;

use super::{
    tokentype::TokenType::{self, *},
    Token,
};

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.r#type {
            Plus | Minus | Star | Slash | Dot | Comma | OpeningBracket | ClosingBracket | Equal
            | Caret | Percentage | LessThan | GreaterThan | LessOrEqualTo | GreaterOrEqualTo
            | DoubleEqual | DoubleAnd | DoubleOr | NotEqual | Exclamation | And | Or => {
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
            Comma => write!(f, ","),
            Equal => write!(f, "="),
            Caret => write!(f, "^"),
            Percentage => write!(f, "%"),
            LessThan => write!(f, "<"),
            GreaterThan => write!(f, ">"),
            And => write!(f, "&"),
            Or => write!(f, "|"),

            LessOrEqualTo => write!(f, "<="),
            GreaterOrEqualTo => write!(f, ">="),
            DoubleEqual => write!(f, "=="),
            DoubleAnd => write!(f, "&&"),
            DoubleOr => write!(f, "||"),
            NotEqual => write!(f, "!="),

            Exclamation => write!(f, "!"),

            OpeningBracket => write!(f, "("),
            ClosingBracket => write!(f, ")"),

            Dot => write!(f, "."),
            Literal => write!(f, "<literal>"),
            Identifier(i_type) => write!(f, "<identifier: {:?}>", i_type),
        }
    }
}
