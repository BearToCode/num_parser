mod display;
pub mod tokentype;

use super::out::{
    ErrorType::{self, *},
    EvalResult,
};
use tokentype::TokenType;

#[derive(Debug, Clone)]
pub struct Token {
    pub r#type: TokenType,
    pub length: usize,
    pub value: String,
}

pub type TokenStream = Vec<Token>;

impl Token {
    fn new(r#type: TokenType, length: usize, value: &str) -> Self {
        Self {
            r#type,
            length,
            value: String::from(value),
        }
    }

    fn join_with(&mut self, token: &Token, r#type: TokenType) {
        self.r#type = r#type;
        self.value.extend(token.value.chars());
        self.length += token.length;
    }
}

/// Builds a stream of tokens.
fn build_stream(source: String) -> EvalResult<TokenStream> {
    let mut stream: TokenStream = vec![];
    let mut content_iter = source.chars();

    while let Some(c) = content_iter.next() {
        stream.push(tokenize(&c)?);
    }

    stream = join_literals(&stream)?;

    Ok(stream)
}

fn remove_whitespaces(string: &mut String) {
    string.replace(" ", "");
}

/// Joins all identifiers.
fn join_identifiers(stream: &TokenStream) -> EvalResult<TokenStream> {
    let mut joined_stream: TokenStream = vec![];

    let mut is_previous_identifier: bool = false;
    // Iterate over the stream and join any literal
    for token in stream {
        let is_identifier = token.r#type == TokenType::Identifier;

        if is_identifier && is_previous_identifier {
            // Join with the previous token and remove the current one.
            let previous_token = joined_stream.last_mut().unwrap();
            previous_token.join_with(token, TokenType::Identifier);
        } else {
            joined_stream.push(token.clone());
        }

        is_previous_identifier = is_identifier;
    }

    Ok(joined_stream)
}

/// Joins numbers handling commas.
fn join_literals(stream: &TokenStream) -> EvalResult<TokenStream> {
    let mut joined_stream: TokenStream = vec![];

    let mut is_previous_literal: bool = false;
    // This is reset for every number
    let mut comma_found = false;
    // Iterate over the stream and join any literal
    for token in stream {
        let is_literal = token.r#type == TokenType::Literal;
        let is_comma = token.r#type == TokenType::Comma;

        if is_literal || is_comma {
            if is_comma {
                if comma_found {
                    return Err(ErrorType::InvalidTokenAtPosition {
                        token: token.r#type,
                    }); // TODO:
                }
                comma_found = true;
            }

            if is_previous_literal {
                // Join with the previous token and remove the current one.
                let previous_token = joined_stream.last_mut().unwrap();
                previous_token.join_with(token, TokenType::Literal);
            } else {
                joined_stream.push(token.clone());
            }
        } else {
            comma_found = false;

            joined_stream.push(token.clone());
        }

        is_previous_literal = is_literal;
    }

    Ok(joined_stream)
}

/// Returns the char corresponding token.
fn tokenize(character: &char) -> EvalResult<Token> {
    Ok(match character {
        '+' => Token::new(TokenType::Plus, 1, ""),
        '-' => Token::new(TokenType::Minus, 1, ""),
        '*' => Token::new(TokenType::Star, 1, ""),
        '/' => Token::new(TokenType::Slash, 1, ""),
        '.' => Token::new(TokenType::Comma, 1, ""),
        other => {
            let as_string = format!("{}", other);
            if other.is_numeric() {
                Token::new(TokenType::Literal, 1, &as_string)
            } else if other.is_alphabetic() {
                Token::new(TokenType::Identifier, 1, &as_string)
            } else {
                return Err(ErrorType::UnknownToken {
                    token: String::from(&as_string),
                });
            }
        }
    })
}
