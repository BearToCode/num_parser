use super::expr::Expression;
use super::function::*;
use super::token::{tokentype::TokenType, Token, TokenStream};

/// An expression is a node in the expression tree.
type Node = Expression;

/// Builds an expression tree, effectively parsing the token stream.
pub fn build_tree(stream: TokenStream) {}
