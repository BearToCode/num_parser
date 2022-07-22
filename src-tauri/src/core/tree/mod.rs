// use evalexpr::ValueType;

// use super::expr::Expression;
// use super::function::*;
// use super::out::{EvalResult, ErrorType};
// use super::token::{tokentype::TokenType, Token, TokenStream};

// /// An expression is a node in the expression tree.
// type Node = Expression;

// /// Builds an expression tree, effectively parsing the token stream.
// pub fn build_tree(stream: TokenStream) {
//     // Sort by precedence
//     let sorted_node_tokens_iter = sort_node_tokens(&stream).iter();
//     // For every token, starting from the token node
//     // with the lowest precedence:
//     // TODO: NOT USE SOME BUT ALWAYS RETRIEVE THE LOWEST
//     // PRECEDENCE, AS MULTIPLE TOKENS MIGHT BE POPPED
//     // IN THE SAME ITERATION (IF NOT ALL).
//     while let Some(token_info) = sorted_node_tokens_iter.next() {
//         let token = token_info.token;
//         //   Match between all the possible expressions:
//         //     - If value, do nothing.
//         //     - If it is an operator, get the previous
//         //     and successive tokens, which, once built,
//         //     will return the necessary value and build
//         //     their nodes.
//         //     - If it is a function call, retrieve all parameters
//         //     and build their nodes.
//         match token.r#type {
//             TokenType::Literal {

//             }
//         }

//     }
// }

// // IS THIS REQUIRED?
// fn get_maximum_depth(stream: &TokenStream) -> EvalResult<u8> {
//     let mut depth = 0;
//     let mut max_depth = 0;
//     for token in stream {
//         match token.r#type {
//             TokenType::OpeningBracket => depth += 1,
//             TokenType::ClosingBracket => {
//                 depth -= 1;
//                 if depth < 0 {
//                     // Also check for invalid brackets
//                     return Err(ErrorType::InvalidClosingBracket);
//                 }
//             }
//             _ => ()
//         }
//         max_depth = std::cmp::max(depth, max_depth);
//     }
//     Ok(max_depth as u8)
// }

// struct TokenInfo {
//     pub token: Token,
//     pub position: u16,
//     pub precedence: u16,
// }

// /// Sorts all possible
// fn sort_node_tokens(stream: &TokenStream) -> Vec<TokenInfo> {
//     let mut sorted = vec![];
//     let mut depth = 0;
//     for (position, token) in stream.iter().enumerate() {
//         if token.r#type == TokenType::OpeningBracket {
//             depth += 1;
//         } else if token.r#type == TokenType::ClosingBracket {
//             depth -= 1;
//         }else if token.r#type.is_expression() {
//             let precedence = TokenType::max_precedence() * depth +
//                 token.r#type.precedence();
//             sorted.push(TokenInfo {
//                 token: *token,
//                 position: position as u16,
//                 precedence
//             });
//         }
//     }
//     sorted.sort_by_key(|v| v.precedence);
//     sorted
// }

// fn create_node(token: &Token) -> EvalResult<Node> {

// }

// fn get_previous_token() -> Option<u16> {

// }

// fn get_next_token() {

// }
