use crate::{
    expr::Expression,
    function::builtin,
    out::{ErrorType, EvalResult},
    token::{tokentype::TokenType, Token, TokenStream},
    value::Value,
};

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

/// Consumes the token with the provided stream position and builds a node. It finds
/// eventual required node parameters inside the same vectors within the specified range.
fn create_node(
    sorted_node_tokens: &mut Vec<TokenInfo>,
    stream: &TokenStream,
    position: Option<usize>,
    range: (usize, usize),
) -> EvalResult<Node> {
    let index = match position {
        Some(value) => {
            // Find the node in the sorted ones with the corresponding index
            match sorted_node_tokens.iter().position(|x| x.position == value) {
                Some(position) => position,
                None => {
                    return Err(ErrorType::InternalError {
                        message: String::from("trying to remove non-existing token"),
                    });
                }
            }
        }
        None => 0,
    };

    if sorted_node_tokens.len() == 0 {
        if position == None {
            // First iteration, so input is empty
            return Ok(Node::Value(Value::Int(0)));
        } else {
            // This is an error
            return Err(ErrorType::InternalError {
                message: String::from("trying to remove non-existing token"),
            });
        }
    }

    let token_info = sorted_node_tokens.remove(index);

    // Get the node type.
    if token_info.token.r#type.is_binary_operator() && token_info.token.r#type.is_unary_operator() {
        // In this case we need to check for both unary and binary
        match build_binary_operator(sorted_node_tokens, stream, &token_info, range) {
            Ok(node) => return Ok(node),
            Err(_) => {
                return Ok(build_unary_operator(
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
            TokenType::VariableIdentifier => {
                match builtin::consts(&token_info.token.value) {
                    Some(value) => return Ok(Node::Value(value)),
                    // TODO: CONTEXT
                    None => unimplemented!(),
                }
            }
            TokenType::FunctionIdentifier => {
                match builtin::functions(&token_info.token.value) {
                    Some(func) => {
                        return Ok(Node::Func(
                            func,
                            Box::new(get_function_parameters(
                                sorted_node_tokens,
                                stream,
                                &token_info,
                            )?),
                        ))
                    }
                    // TODO: CONTEXT
                    None => unimplemented!(),
                }
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
            match get_lowest_precedence_node_in_range(
                sorted_node_tokens,
                stream,
                (range.0, token_info.position),
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
            // Previous node
            match get_lowest_precedence_node_in_range(
                sorted_node_tokens,
                stream,
                (range.0, token_info.position),
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
            // Successive node
            match get_lowest_precedence_node_in_range(
                sorted_node_tokens,
                stream,
                (token_info.position + 1, range.1),
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
                        Some(value.position),
                        range,
                    )?)
                }
                None => return Err(ErrorType::EmptyBrackets),
            },
        )
    }
}

/// Returns a node with can contains multiple values as a vector.
fn get_function_parameters(
    sorted_node_tokens: &mut Vec<TokenInfo>,
    stream: &TokenStream,
    func_token: &TokenInfo,
) -> EvalResult<Node> {
    let func_pos = func_token.position;
    // Check if in range
    if func_pos + 1 < stream.len() {
        // Check for bracket
        if stream[func_pos + 1].r#type == TokenType::OpeningBracket {
            // Builds the node inside the brackets
            match get_lowest_precedence_node_in_range(
                sorted_node_tokens,
                stream,
                (
                    func_pos + 1,
                    get_corresponding_closing_bracket(stream, func_pos + 1)?,
                ),
            )? {
                Some(node) => Ok(node),
                None => Err(ErrorType::MissingFunctionParameters {
                    func_name: func_token.token.value.clone(),
                }),
            }
        } else {
            // See if there is an available token to be built after the function token
            match sorted_node_tokens
                .iter()
                .find(|x| x.position == func_pos + 1)
            {
                Some(token_info) => {
                    // an available token has been found, see if it a literal, a variable or a function
                    match token_info.token.r#type {
                        TokenType::Literal
                        | TokenType::VariableIdentifier
                        | TokenType::FunctionIdentifier => Ok(create_node(
                            sorted_node_tokens,
                            stream,
                            Some(token_info.position),
                            (token_info.position, token_info.position + 1),
                        )?),
                        // this token cannot be used as a function parameter,
                        // we only want implicit tokens that do not require other nodes,
                        // like: sin2 = sin(2), sinpi = sin(pi), sincos(2) = sin(cos(2))
                        _ => Err(ErrorType::MissingFunctionParameters {
                            func_name: func_token.token.value.clone(),
                        }),
                    }
                }
                None => {
                    // No available token
                    Err(ErrorType::MissingFunctionParameters {
                        func_name: func_token.token.value.clone(),
                    })
                }
            }
        }
    } else {
        Err(ErrorType::MissingFunctionParameters {
            func_name: func_token.token.value.clone(),
        })
    }
}

fn get_corresponding_closing_bracket(
    stream: &TokenStream,
    opening_bracket_pos: usize,
) -> EvalResult<usize> {
    let mut index = opening_bracket_pos + 1;
    let mut current_depth = 0;
    while index < stream.len() {
        let token: &Token = &stream[index];

        if token.r#type == TokenType::ClosingBracket {
            if current_depth == 0 {
                return Ok(index.try_into().unwrap());
            }
            current_depth -= 1;
        } else if token.r#type == TokenType::OpeningBracket {
            current_depth += 1;
        }
        index += 1;
    }
    Err(ErrorType::MissingClosingBracket)
}
