mod display;
pub mod tokentype;

use crate::{
    context::Context,
    function::builtin,
    out::{ErrorType, EvalResult},
    token::tokentype::TokenType,
};

use self::tokentype::IdentifierType;
use itertools::Itertools;

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
pub fn build_stream(mut source: String, context: &Context) -> EvalResult<TokenStream> {
    source = remove_whitespaces(&source);

    let mut stream: TokenStream = vec![];
    let mut content_iter = source.chars();

    while let Some(c) = content_iter.next() {
        stream.push(tokenize(&c)?);
    }

    stream = join_operators(&stream);
    stream = join_identifiers(&stream)?;
    stream = join_literals(&stream)?;
    stream = format_identifiers(&stream, context);
    stream = predict_unknown_identifiers(&stream);
    stream = add_implicit_brackets(&stream)?;
    stream = add_implicit_multiplications(&stream);

    Ok(stream)
}

fn join_operators(stream: &TokenStream) -> TokenStream {
    fn find_and_join(
        stream: &TokenStream,
        pattern: Vec<TokenType>,
        replacement: TokenType,
    ) -> TokenStream {
        let mut out_v = vec![];

        let mut start_index = 0;
        let mut end_index = pattern.len();

        let mut stream_as_iter = stream.iter();

        while end_index <= stream.len() {
            let slice = &stream[start_index..end_index]
                .iter()
                .map(|token| token.r#type)
                .collect::<Vec<TokenType>>();

            if *slice == pattern {
                out_v.push(Token::new(replacement, pattern.len(), ""));

                for _ in 0..pattern.len() {
                    // Advance the iterator, consuming the replaced tokens.
                    stream_as_iter.next();
                }

                start_index += pattern.len();
                end_index += pattern.len();
            } else {
                out_v.push(stream[start_index].clone());
                stream_as_iter.next();

                start_index += 1;
                end_index += 1;
            }
        }

        // Add the remaining data
        for token in stream_as_iter {
            out_v.push(token.clone());
        }

        out_v
    }

    use TokenType::*;

    let mut new_stream = stream.clone();
    new_stream = find_and_join(&new_stream, vec![LessThan, Equal], LessOrEqualTo);
    new_stream = find_and_join(&new_stream, vec![GreaterThan, Equal], GreaterOrEqualTo);
    new_stream = find_and_join(&new_stream, vec![Equal, Equal], DoubleEqual);
    new_stream = find_and_join(&new_stream, vec![Exclamation, Equal], NotEqual);
    new_stream = find_and_join(&new_stream, vec![And, And], DoubleAnd);
    new_stream = find_and_join(&new_stream, vec![Or, Or], DoubleOr);

    new_stream
}

fn predict_unknown_identifiers(stream: &TokenStream) -> TokenStream {
    // Return if empty.
    if stream.len() == 0 {
        return stream.clone();
    }

    let mut out_stream = vec![];

    let stream_as_iter = stream.iter();

    let mut convert = |prev: &Token, next: Option<&Token>| {
        let next_is_bracket = match next {
            Some(token) => token.r#type == TokenType::OpeningBracket,
            None => false,
        };

        if prev.r#type == TokenType::Identifier(IdentifierType::Unknown) {
            if next_is_bracket {
                // Categorize as function
                out_stream.push(Token::new(
                    TokenType::Identifier(IdentifierType::Function),
                    prev.value.len(),
                    &prev.value[..],
                ));
            } else {
                // Categorize as var
                out_stream.push(Token::new(
                    TokenType::Identifier(IdentifierType::Var),
                    prev.value.len(),
                    &prev.value[..],
                ));
            }
        } else {
            out_stream.push(prev.clone());
        }
    };

    // Convert unknown identifiers to function if they are followed by a parenthesis
    for (prev, next) in stream_as_iter.tuple_windows() {
        convert(prev, Some(next));
    }

    // Add last token, not included in the previous identifier
    let last_token = stream.iter().last().unwrap();
    convert(last_token, None);

    out_stream
}

fn add_implicit_brackets(stream: &TokenStream) -> EvalResult<TokenStream> {
    // Return if empty.
    if stream.len() == 0 {
        return Ok(stream.clone());
    }

    let mut out_stream = vec![];

    let mut skip_iteration = false;

    let stream_as_iter = stream.iter();
    // Add implicit brackets for all functions not followed by an opening bracket
    for (prev, next) in stream_as_iter.tuple_windows() {
        if skip_iteration {
            skip_iteration = false;
            continue;
        }

        if prev.r#type == TokenType::Identifier(IdentifierType::Function)
            && next.r#type != TokenType::OpeningBracket
        {
            if next.r#type == TokenType::Literal
                || next.r#type == TokenType::Identifier(IdentifierType::Var)
            {
                out_stream.push(prev.clone());
                out_stream.push(Token::new(TokenType::OpeningBracket, 1, ""));
                out_stream.push(next.clone());
                out_stream.push(Token::new(TokenType::ClosingBracket, 1, ""));
            } else {
                return Err(ErrorType::MissingFunctionParameters {
                    func_name: prev.value.clone(),
                });
            }
        } else {
            out_stream.push(prev.clone());
        }
    }

    if !skip_iteration {
        // Push last item if it was not included during the previous iteration.
        out_stream.push(stream.iter().last().unwrap().clone());
    }
    Ok(out_stream)
}

fn format_identifiers(stream: &TokenStream, context: &Context) -> TokenStream {
    let mut out_stream = vec![];

    let mut stream_as_iter = stream.iter();
    while let Some(token) = stream_as_iter.next() {
        if token.r#type == TokenType::Identifier(IdentifierType::Unknown) {
            let content = token.value.clone();
            let splitted = split_into_identifiers(content, context);
            for (i, i_type) in splitted {
                out_stream.push(Token::new(TokenType::Identifier(i_type), i.len(), &i[..]));
            }
        } else {
            out_stream.push(token.clone());
        }
    }
    out_stream
}

/// Given a string, returns a vector with all identified vars and function.
pub fn split_into_identifiers(input: String, context: &Context) -> Vec<(String, IdentifierType)> {
    let mut out = vec![];

    /// Given a string tries to find a match from all possible categories.
    fn try_to_categorize(
        sorted_patterns: &Vec<(IdentifierType, Vec<&str>)>,
        candidate: &String,
    ) -> Option<IdentifierType> {
        let mut patterns_as_iter = sorted_patterns.iter();
        while let Some((i_type, patterns)) = patterns_as_iter.next() {
            for pattern in patterns {
                if &&candidate[..] == pattern {
                    return Some(*i_type);
                }
            }
        }
        None
    }

    // In order of priority:
    // Built-in functions
    // Built-in consts
    // User-defined functions
    // User-defined vars
    let patterns = vec![
        (
            IdentifierType::Function,
            builtin::BUILT_IN_FUNCTIONS
                .iter()
                .map(|x| x.func_identifier)
                .collect::<Vec<&str>>(),
        ),
        (
            IdentifierType::Var,
            builtin::CONSTANTS
                .iter()
                .map(|x| x.0)
                .cloned()
                .collect::<Vec<&str>>(),
        ),
        (
            IdentifierType::Function,
            context
                .functions
                .iter()
                .map(|x| &x.0[..])
                .collect::<Vec<&str>>(),
        ),
        (
            IdentifierType::Var,
            context
                .variables
                .iter()
                .map(|x| &x.0[..])
                .collect::<Vec<&str>>(),
        ),
    ];

    let mut current = "".to_owned();

    for char in input.chars() {
        current.push(char);
        if let Some(identifier) = try_to_categorize(&patterns, &current) {
            out.push((current, identifier));
            current = "".to_owned();
        }
    }

    if current != "" {
        out.push((current, IdentifierType::Unknown));
    }

    out
}

fn add_implicit_multiplications(stream: &TokenStream) -> TokenStream {
    // Return if empty.
    if stream.len() == 0 {
        return stream.clone();
    }

    let mut out_stream: TokenStream = vec![stream[0].clone()];

    let mut previous_token_type = stream[0].r#type;
    let mut index = 1;

    while index < stream.len() {
        let current_type = stream[index].r#type;

        use IdentifierType::*;
        use TokenType::*;
        // Add token '*' between:
        // literal-bracket: 2(4) or (4)2
        if previous_token_type == Literal               && current_type == OpeningBracket
            || previous_token_type == ClosingBracket        && current_type == Literal
            // literal-literal
            || previous_token_type == Literal               && current_type == Literal
            // bracket-bracket: (2)(4)
            || previous_token_type == ClosingBracket        && current_type == OpeningBracket
            // bracket-any identifier: (2)pi
            || previous_token_type == ClosingBracket        && matches!(current_type, Identifier(_))
            // var-bracket
            || previous_token_type == Identifier(Var)       && current_type == OpeningBracket
            // literal-any identifier: 2pi
            || previous_token_type == Literal               && matches!(current_type, Identifier(_))
            // any identifier-literal
            || matches!(previous_token_type, Identifier(_)) && current_type == Literal
            // any identifier-any identifier
            || matches!(previous_token_type, Identifier(_)) && matches!(current_type, Identifier(_))
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
        let is_identifier = token.r#type == TokenType::Identifier(IdentifierType::Unknown);

        if is_identifier && is_previous_identifier {
            // Join with the previous token and avoid pushing the current one.
            let previous_token = joined_stream.last_mut().unwrap();
            previous_token.join_with(token, TokenType::Identifier(IdentifierType::Unknown));
        } else {
            joined_stream.push(token.clone());
        }

        is_previous_identifier = is_identifier;
    }

    Ok(joined_stream)
}

/// Join numbers handling commas.
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
                    return Err(ErrorType::InvalidTokenPosition {
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
        ',' => Token::new(TokenType::Comma, 1, ""),
        '=' => Token::new(TokenType::Equal, 1, ""),
        '^' => Token::new(TokenType::Caret, 1, ""),
        '%' => Token::new(TokenType::Percentage, 1, ""),
        '<' => Token::new(TokenType::LessThan, 1, ""),
        '>' => Token::new(TokenType::GreaterOrEqualTo, 1, ""),
        '&' => Token::new(TokenType::And, 1, ""),
        '|' => Token::new(TokenType::Or, 1, ""),
        '!' => Token::new(TokenType::Exclamation, 1, ""),

        '.' => Token::new(TokenType::Dot, 1, "."),

        '(' => Token::new(TokenType::OpeningBracket, 1, ""),
        ')' => Token::new(TokenType::ClosingBracket, 1, ""),
        other => {
            let as_string = format!("{}", other);
            if other.is_numeric() {
                Token::new(TokenType::Literal, 1, &as_string)
            } else if other.is_alphabetic() {
                Token::new(
                    TokenType::Identifier(IdentifierType::Unknown),
                    1,
                    &as_string,
                )
            } else {
                return Err(ErrorType::UnknownToken {
                    token: String::from(&as_string),
                });
            }
        }
    })
}
