mod display;
pub mod tokentype;

use super::out::{ErrorType, EvalResult};
use tokentype::TokenType;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    /// The generic token type.
    pub r#type: TokenType,
    /// The token size inside the original input.
    pub length: usize,
    /// The token content. Only useful for literals and identifiers.
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
pub fn build_stream(mut source: String) -> EvalResult<TokenStream> {
    source = remove_whitespaces(&source);

    let mut stream: TokenStream = vec![];
    let mut content_iter = source.chars();

    while let Some(c) = content_iter.next() {
        stream.push(tokenize(&c)?);
    }

    stream = join_identifiers(&stream)?;
    stream = categorize_identifiers(&stream)?;
    stream = join_literals(&stream)?;
    stream = add_implicit_multiplications(&stream);

    Ok(stream)
}

fn add_implicit_multiplications(stream: &TokenStream) -> TokenStream {
    // Add token '*' between:
    // literal-bracket: 2(4) or (4)2
    // bracket-bracket: (2)(4)
    // variable-bracket: pi(2) or (2)pi
    // variable-literal: 2pi or pi2

    // Return if empty.
    if stream.len() == 0 {
        return stream.clone();
    }

    let mut out_stream: TokenStream = vec![stream[0].clone()];

    let mut previous_token_type = stream[0].r#type;
    let mut index = 1;

    while index < stream.len() {
        let current_type = stream[index].r#type;

        if previous_token_type == TokenType::Literal && current_type == TokenType::OpeningBracket
            || previous_token_type == TokenType::ClosingBracket
                && current_type == TokenType::Literal
            || previous_token_type == TokenType::ClosingBracket
                && current_type == TokenType::OpeningBracket
            || previous_token_type == TokenType::VariableIdentifier
                && current_type == TokenType::OpeningBracket
            || previous_token_type == TokenType::ClosingBracket
                && current_type == TokenType::VariableIdentifier
            || previous_token_type == TokenType::Literal
                && current_type == TokenType::VariableIdentifier
            || previous_token_type == TokenType::VariableIdentifier
                && current_type == TokenType::Literal
        {
            out_stream.push(Token::new(TokenType::Star, 1, ""));
        }

        out_stream.push(stream[index].clone());
        previous_token_type = current_type;
        index += 1;
    }

    out_stream
}

fn remove_whitespaces(string: &String) -> String {
    string.replace(" ", "")
}

/// Joins all identifiers.
fn join_identifiers(stream: &TokenStream) -> EvalResult<TokenStream> {
    let mut joined_stream: TokenStream = vec![];

    let mut is_previous_identifier: bool = false;
    // Iterate over the stream and join any literal
    for token in stream {
        let is_identifier = token.r#type == TokenType::UnknownIdentifier;

        if is_identifier && is_previous_identifier {
            // Join with the previous token and avoid pushing the current one.
            let previous_token = joined_stream.last_mut().unwrap();
            previous_token.join_with(token, TokenType::UnknownIdentifier);
        } else {
            joined_stream.push(token.clone());
        }

        is_previous_identifier = is_identifier;
    }

    Ok(joined_stream)
}

/// Determine if the identifier is a value, a function or a variable.
fn categorize_identifiers(stream: &TokenStream) -> EvalResult<TokenStream> {
    Ok(stream
        .iter()
        .map(|x| {
            if x.r#type == TokenType::UnknownIdentifier {
                match &x.value[..] {
                    "true" => Token::new(TokenType::Literal, 4, "true"),
                    "false" => Token::new(TokenType::Literal, 5, "false"),
                    // TODO:
                    _ => unimplemented!(),
                }
            } else {
                x.clone()
            }
        })
        .collect())
}

/// Joins numbers handling commas.
fn join_literals(stream: &TokenStream) -> EvalResult<TokenStream> {
    let mut joined_stream: TokenStream = vec![];

    let mut is_previous_literal: bool = false;
    // This is reset for every number
    let mut comma_found = false;
    // Iterate over the stream and join any literal
    for token in stream {
        let is_comma = token.r#type == TokenType::Dot;
        let is_literal = token.r#type == TokenType::Literal || is_comma;

        if is_literal || is_comma {
            if is_comma {
                // Allow only one comma
                if comma_found {
                    return Err(ErrorType::InvalidTokenAtPosition {
                        token: token.r#type,
                    });
                }
                comma_found = true;
            }

            if is_previous_literal {
                // Join with the previous token and avoid pushing the current one.
                let previous_token = joined_stream.last_mut().unwrap();
                previous_token.join_with(token, TokenType::Literal);
            } else {
                joined_stream.push(token.clone());
            }
        } else {
            // Reset temp vars if it is not a literal
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
        '.' => Token::new(TokenType::Dot, 1, "."),
        '(' => Token::new(TokenType::OpeningBracket, 1, ""),
        ')' => Token::new(TokenType::ClosingBracket, 1, ""),
        other => {
            let as_string = format!("{}", other);
            if other.is_numeric() {
                Token::new(TokenType::Literal, 1, &as_string)
            } else if other.is_alphabetic() {
                // TODO: VECTORS (CHECK COMMAS)

                // TODO: TO FUNCTION OR IDENTIFIER
                // TODO: IMAGINARY NUMBERS

                // TODO: CREATE LIST OF RESERVED KEYWORDS TO NOT USE AS STATEMENTS

                // SHOULD NOT BE DONE HERE
                // unimplemented!()
                Token::new(TokenType::UnknownIdentifier, 1, &as_string)
            } else {
                return Err(ErrorType::UnknownToken {
                    token: String::from(&as_string),
                });
            }
        }
    })
}
