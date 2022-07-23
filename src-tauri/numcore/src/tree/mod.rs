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
    check_brackets(&stream)?;
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
#[derive(Clone, PartialEq)]
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
    sorted.sort_by_key(|v| (v.depth, v.precedence));
    Ok(sorted)
}

/// This function consumes the element at the provided index of the iterator and builds a node. It finds
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
        match build_unary_operator(sorted_node_tokens, stream, &token_info) {
            Ok(node) => return Ok(node),
            Err(_) => {
                return Ok(build_binary_operator(
                    sorted_node_tokens,
                    stream,
                    &token_info,
                )?)
            }
        }
    } else if token_info.token.r#type.is_binary_operator() {
        // Try just binary
        return Ok(build_binary_operator(
            sorted_node_tokens,
            stream,
            &token_info,
        )?);
    } else if token_info.token.r#type.is_unary_operator() {
        // Try just unary
        return Ok(build_unary_operator(
            sorted_node_tokens,
            stream,
            &token_info,
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
) -> EvalResult<Node> {
    Ok(Node::Unary(
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

/// Builds a binary operator with the provided data.
fn build_binary_operator(
    sorted_node_tokens: &mut Vec<TokenInfo>,
    stream: &TokenStream,
    token_info: &TokenInfo,
) -> EvalResult<Node> {
    Ok(Node::Binary(
        Box::new(
            match get_previous_node(sorted_node_tokens, stream, &token_info)? {
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
    ))
}

/// Returns, if it exists, the node before the position specified.
fn get_previous_node(
    sorted_node_tokens: &mut Vec<TokenInfo>,
    stream: &TokenStream,
    pivot_token_info: &TokenInfo,
) -> EvalResult<Option<Node>> {
    // If there is a valid node-token, build it and return it, otherwise return None

    if pivot_token_info.position == 0 {
        return Ok(None);
    }

    // There can either be:
    Ok(match stream[pivot_token_info.position - 1].r#type {
        // an opening bracket,
        TokenType::OpeningBracket => None,
        // a value,
        // a variable,
        TokenType::Literal | TokenType::VariableIdentifier => Some(create_node(
            sorted_node_tokens,
            stream,
            pivot_token_info.position - 1,
        )?),
        // a closing bracket
        TokenType::ClosingBracket => {
            // In this case, we need to get the corresponding opening bracket
            let opening_bracket_index = get_corresponding_opening_bracket(
                stream,
                pivot_token_info.position - 1,
                pivot_token_info.depth as i16,
            )?;
            if opening_bracket_index != 0 {
                // Now check if the brackets were delimiting a function call.
                // Search for a function at position index-1
                match sorted_node_tokens.iter().position(|x| {
                    x.position == opening_bracket_index - 1
                        && x.token.r#type == TokenType::FunctionIdentifier
                }) {
                    // Build the function node if it found
                    Some(index) => Some(create_node(sorted_node_tokens, stream, index)?),
                    // Otherwise find the lowest precedence node in the brackets.
                    None => Some(create_node(
                        sorted_node_tokens,
                        stream,
                        get_lowest_precedence_node_token_index_in_range(
                            sorted_node_tokens,
                            opening_bracket_index,
                            pivot_token_info.position - 1,
                        )?,
                    )?),
                }
            } else {
                None
            }
        }
        other => return Err(ErrorType::InvalidTokenAtPosition { token: other }),
    })

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

/// Get the lowest node-token index in the range. The range is start-inclusive, end-exclusive.
fn get_lowest_precedence_node_token_index_in_range(
    sorted_node_tokens: &Vec<TokenInfo>,
    start: usize,
    end: usize,
) -> EvalResult<usize> {
    let candidates: Vec<TokenInfo> = sorted_node_tokens
        .iter()
        .filter(|&x| x.position >= start && x.position < end)
        .cloned()
        .collect();

    if candidates.len() == 0 {
        Err(ErrorType::EmptyBrackets)
    } else {
        Ok(match candidates.iter().min_by_key(|&x| x.precedence) {
            Some(value) => sorted_node_tokens.iter().position(|x| x == value).unwrap(),
            None => return Err(ErrorType::EmptyBrackets),
        })
    }
}

fn get_corresponding_opening_bracket(
    stream: &TokenStream,
    start: usize,
    depth_at_start: i16,
) -> EvalResult<usize> {
    let mut index = (start - 1) as i16;
    let mut current_depth = depth_at_start;
    while index >= 0 {
        let token: &Token = &stream[index as usize];

        if token.r#type == TokenType::OpeningBracket {
            if current_depth == depth_at_start {
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
