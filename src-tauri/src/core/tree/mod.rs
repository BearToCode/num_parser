use std::slice::Iter;

use super::expr::Expression;
use super::function::*;
use super::out::{ErrorType, EvalResult};
use super::token::{tokentype::TokenType, Token, TokenStream};
use super::value::Value;

/// An expression is a node in the expression tree.
type Node = Expression;

/// Builds an expression tree, effectively parsing the token stream.
pub fn build_tree(stream: TokenStream) -> EvalResult<Node> {
    // Sort by precedence
    let mut sorted_node_tokens = sort_node_tokens(&stream)?;
    // For every token, starting from the token node
    // with the lowest precedence:

    //   Match between all the possible expressions:
    //     - If literal, do nothing.
    //     - If it is an operator, get the previous
    //     and successive tokens, which, once built,
    //     will return the necessary value and build
    //     their nodes.
    //     - If it is a function call, retrieve all parameters
    //     and build their nodes.
    Ok(create_node(&mut sorted_node_tokens, &stream, 0)?)
}

// IS THIS REQUIRED?
fn check_brackets(stream: &TokenStream) -> EvalResult<()> {
    let mut depth = 0;
    for token in stream {
        match token.r#type {
            TokenType::OpeningBracket => depth += 1,
            TokenType::ClosingBracket => {
                depth -= 1;
                if depth < 0 {
                    // Also check for invalid brackets
                    return Err(ErrorType::InvalidClosingBracket);
                }
            }
            _ => (),
        }
    }
    match depth {
        0 => Ok(()),
        _ => return Err(ErrorType::MissingClosingBracket),
    }
}

/// Contains the token info inside the token stream.
struct TokenInfo {
    pub token: Token,
    pub position: usize,
    pub depth: u16,
    pub precedence: u16,
}

/// Sorts all possible tokens that create nodes.
fn sort_node_tokens(stream: &TokenStream) -> EvalResult<Vec<TokenInfo>> {
    let mut sorted = vec![];
    let mut depth = 0;
    for (position, token) in stream.iter().enumerate() {
        if token.r#type == TokenType::OpeningBracket {
            depth += 1;
        } else if token.r#type == TokenType::ClosingBracket {
            depth -= 1;
        } else if token.r#type.is_expression() {
            let precedence = token.r#type.precedence()?;
            sorted.push(TokenInfo {
                token: token.clone(),
                position: position,
                depth,
                precedence,
            });
        }
    }
    sorted.sort_by_key(|v| (v.depth, v.precedence));
    Ok(sorted)
}

/// This function consumes the first element of the iterator and build a node. It finds
/// eventual required node parameters inside the iterator.
fn create_node(
    sorted_node_tokens: &mut Vec<TokenInfo>,
    stream: &TokenStream,
    index: usize,
) -> EvalResult<Node> {
    let token_info = sorted_node_tokens.remove(index);
    // Get the node type.
    if token_info.token.r#type.is_binary_operator() && token_info.token.r#type.is_unary_operator() {
        // In this case we need to check for both unary and binary
        match get_previous_node(sorted_node_tokens, stream, &token_info) {
            // Try with binary
            Some(previous_node) => {
                return Ok(Node::Binary(
                    Box::new(previous_node),
                    token_info.token.r#type,
                    Box::new({
                        match get_next_node(sorted_node_tokens, stream, &token_info) {
                            Some(next_node) => next_node,
                            None => {
                                return Err(ErrorType::MissingOperatorArgument {
                                    token: token_info.token.r#type,
                                })
                            }
                        }
                    }),
                ))
            }
            // Or with unary
            None => {
                return Ok(Node::Unary(
                    token_info.token.r#type,
                    Box::new(
                        match get_next_node(sorted_node_tokens, stream, &token_info) {
                            Some(next_node) => next_node,
                            None => {
                                return Err(ErrorType::MissingOperatorArgument {
                                    token: token_info.token.r#type,
                                })
                            }
                        },
                    ),
                ))
            }
        }
    } else if token_info.token.r#type.is_binary_operator() {
        // Try just binary
        return Ok(Node::Binary(
            Box::new(
                match get_previous_node(sorted_node_tokens, stream, &token_info) {
                    Some(previous_node) => previous_node,
                    None => {
                        return Err(ErrorType::MissingOperatorArgument {
                            token: token_info.token.r#type,
                        })
                    }
                },
            ),
            token_info.token.r#type,
            Box::new(
                match get_next_node(sorted_node_tokens, stream, &token_info) {
                    Some(next_node) => next_node,
                    None => {
                        return Err(ErrorType::MissingOperatorArgument {
                            token: token_info.token.r#type,
                        })
                    }
                },
            ),
        ));
    } else if token_info.token.r#type.is_unary_operator() {
        // Try just unary
        return Ok(Node::Unary(
            token_info.token.r#type,
            Box::new(
                match get_next_node(sorted_node_tokens, stream, &token_info) {
                    Some(next_node) => next_node,
                    None => {
                        return Err(ErrorType::MissingOperatorArgument {
                            token: token_info.token.r#type,
                        })
                    }
                },
            ),
        ));
    } else {
        // Match for functions, variables or literals.
        match token_info.token.r#type {
            TokenType::Literal => {
                return Ok(Node::Value(Value::from_string(token_info.token.value)?))
            }
            // TODO:
            _ => unimplemented!(),
        }
    }
}

/// Returns, if it exists, the token before the position specified.
fn get_previous_node(
    sorted_node_tokens: &Vec<TokenInfo>,
    stream: &TokenStream,
    pivot_token_info: &TokenInfo,
) -> Option<Node> {
    // If there is a valid node-token, build it and return it, otherwise return None

    if pivot_token_info.position == 0 {
        return None;
    }

    unimplemented!()

    // match stream[pivot_token_info.position - 1].r#type {
    // 	TokenType::ClosingBracket
    // }

    // let starting_depth = pivot_token_info.depth;
}

/// Returns, if it exists, the token after the position specified.
fn get_next_node(
    sorted_node_tokens: &Vec<TokenInfo>,
    stream: &TokenStream,
    pivot_token_info: &TokenInfo,
) -> Option<Node> {
    unimplemented!()
}
