use std::fmt::Display;

use crate::objects::Request;

impl Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Evaluation(_) => write!(f, "evaluation"),
            Self::FuncDeclaration(_, _, _) => write!(f, "function declaration"),
            Self::VarDeclaration(_, _) => write!(f, "variable declaration"),
        }
    }
}
