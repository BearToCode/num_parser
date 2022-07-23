use std::slice::Iter;

use crate::token::tokentype;

use super::expr::Expression;
use super::function::*;
use super::out::{ErrorType, EvalResult};
use super::token::{tokentype::TokenType, Token, TokenStream};
use super::value::Value;

/// An expression is a node in the expression tree.
type Node = Expression;

/// Builds an expression tree, effectively parsing the token stream.
pub fn build_tree(stream: TokenStream) -> EvalResult<Node> {
    check_brackets(&stream)?;
    // Sort by precedence
    let mut sorted_node_tokens = sort_node_tokens(&stream)?;
    println!("SORTED: {:?}", sorted_node_tokens);
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
    Ok(create_node(
        &mut sorted_node_tokens,
        &stream,
        None,
        (0, stream.len()),
    )?)
}

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
#[derive(Clone, PartialEq, Debug)]
struct TokenInfo {
    /// The contained token.
    pub token: Token,
    /// The token position inside the stream.
    pub position: usize,
    /// The brackets depth at the token position.
    pub depth: u16,
    /// The token precedence.
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
    sorted.sort_by_key(|v| (v.depth, v.precedence, -(v.position as i16)));
    Ok(sorted)
}

/// This function consumes the element at the provided index of the iterator and builds a node. It finds
/// eventual required node parameters inside the iterator.
fn create_node(
    sorted_node_tokens: &mut Vec<TokenInfo>,
    stream: &TokenStream,
    index: Option<usize>,
    range: (usize, usize),
) -> EvalResult<Node> {
    let index = match index {
        Some(value) => {
            // Find the node in the sorted ones with the corresponding index
            match sorted_node_tokens.iter().position(|x| x.position == value) {
                Some(position) => position,
                None => {
                    unimplemented!()
                }
            }
        }
        None => 0,
    };

    let token_info = sorted_node_tokens.remove(index);
    println!("REMOVED: {:?}", token_info);

    // Get the node type.
    if token_info.token.r#type.is_binary_operator() && token_info.token.r#type.is_unary_operator() {
        // In this case we need to check for both unary and binary
        match build_unary_operator(sorted_node_tokens, stream, &token_info, range) {
            Ok(node) => return Ok(node),
            Err(_) => {
                return Ok(build_binary_operator(
                    sorted_node_tokens,
                    stream,
                    &token_info,
                    range,
                )?)
            }
        }
    } else if token_info.token.r#type.is_binary_operator() {
        // Try just binary
        return Ok(build_binary_operator(
            sorted_node_tokens,
            stream,
            &token_info,
            range,
        )?);
    } else if token_info.token.r#type.is_unary_operator() {
        // Try just unary
        return Ok(build_unary_operator(
            sorted_node_tokens,
            stream,
            &token_info,
            range,
        )?);
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

/// Builds a unary operator from the provided data.
fn build_unary_operator(
    sorted_node_tokens: &mut Vec<TokenInfo>,
    stream: &TokenStream,
    token_info: &TokenInfo,
    range: (usize, usize),
) -> EvalResult<Node> {
    Ok(Node::Unary(
        token_info.token.r#type,
        Box::new(
            match get_lowest_precedence_successive_node(
                sorted_node_tokens,
                stream,
                &token_info,
                range,
            )? {
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

/// Builds a binary operator with the provided data.
fn build_binary_operator(
    sorted_node_tokens: &mut Vec<TokenInfo>,
    stream: &TokenStream,
    token_info: &TokenInfo,
    range: (usize, usize),
) -> EvalResult<Node> {
    Ok(Node::Binary(
        Box::new(
            match get_lowest_precedence_previous_node(
                sorted_node_tokens,
                stream,
                &token_info,
                range,
            )? {
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
            match get_lowest_precedence_successive_node(
                sorted_node_tokens,
                stream,
                &token_info,
                range,
            )? {
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

/// Returns, if it exists, the node before the position specified.
fn get_lowest_precedence_previous_node(
    sorted_node_tokens: &mut Vec<TokenInfo>,
    stream: &TokenStream,
    pivot_token_info: &TokenInfo,
    range: (usize, usize),
) -> EvalResult<Option<Node>> {
    // If there is a valid node-token, build it and return it, otherwise return None
    get_lowest_precedence_node_in_range(
        sorted_node_tokens,
        stream,
        (range.0, pivot_token_info.position),
    )
}

/// Returns, if it exists, the token after the position specified.
fn get_lowest_precedence_successive_node(
    sorted_node_tokens: &mut Vec<TokenInfo>,
    stream: &TokenStream,
    pivot_token_info: &TokenInfo,
    range: (usize, usize),
) -> EvalResult<Option<Node>> {
    // If there is a valid node-token, build it and return it, otherwise return None
    get_lowest_precedence_node_in_range(
        sorted_node_tokens,
        stream,
        (pivot_token_info.position, range.1),
    )
}

/// Get the lowest precedence node in the range. The range is start-inclusive, end-exclusive.
fn get_lowest_precedence_node_in_range(
    sorted_node_tokens: &mut Vec<TokenInfo>,
    stream: &TokenStream,
    range: (usize, usize),
) -> EvalResult<Option<Node>> {
    let candidates: Vec<TokenInfo> = sorted_node_tokens
        .iter()
        .filter(|&x| x.position >= range.0 && x.position < range.1)
        .cloned()
        .collect();

    if candidates.len() == 0 {
        Ok(None)
    } else {
        Ok(
            match candidates
                .iter()
                .min_by_key(|&x| (x.depth, x.precedence, -(x.position as i16)))
            {
                Some(value) => {
                    // Create the node
                    Some(create_node(
                        sorted_node_tokens,
                        stream,
                        Some(sorted_node_tokens.iter().position(|x| x == value).unwrap()),
                        range,
                    )?)
                }
                None => return Err(ErrorType::EmptyBrackets),
            },
        )
    }
}

fn get_corresponding_opening_bracket(stream: &TokenStream, start: usize) -> EvalResult<usize> {
    let mut index = (start - 1) as i16;
    let mut current_depth = 0;
    while index >= 0 {
        let token: &Token = &stream[index as usize];

        if token.r#type == TokenType::OpeningBracket {
            if current_depth == 0 {
                return Ok(index.try_into().unwrap());
            }
            current_depth -= 1;
        } else if token.r#type == TokenType::ClosingBracket {
            current_depth += 1;
        }
        index -= 1;
    }
    Err(ErrorType::InvalidClosingBracket)
}

fn get_corresponding_closing_bracket(stream: &TokenStream, start: usize) -> EvalResult<usize> {
    let mut index = (start + 1) as i16;
    let mut current_depth = 0;
    while (index as usize) < stream.len() {
        let token: &Token = &stream[index as usize];

        if token.r#type == TokenType::OpeningBracket {
            current_depth -= 1;
        } else if token.r#type == TokenType::ClosingBracket {
            if current_depth == 0 {
                return Ok(index.try_into().unwrap());
            }
            current_depth += 1;
        }
        index += 1;
    }
    Err(ErrorType::MissingClosingBracket)
}
