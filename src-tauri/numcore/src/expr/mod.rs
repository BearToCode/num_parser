use super::{
    function::Function,
    out::{ErrorType, EvalResult},
    token::tokentype::TokenType,
    value::Value,
};

#[derive(Debug)]
pub struct Identifier(String);

// TODO: STATEMENTS

#[derive(Debug)]
pub enum Expression {
    /// A binary operation between two expression.
    Binary(Box<Expression>, TokenType, Box<Expression>),
    /// An unary operation to an expression.
    Unary(TokenType, Box<Expression>),
    /// A variable.
    Var(Identifier),
    /// A function call and its parameters.
    Func(Function, Box<Expression>),
    /// A literal value.
    Value(Value),
    // TODO: An equation.
    // Equation(Box<Expression>, Box<Expression>),
}

impl Expression {
    pub fn eval(&self) -> EvalResult<Value> {
        match self {
            Self::Binary(left_expr, token_type, right_expr) => {
                let left_value = (**left_expr).eval()?;
                let right_value = (**right_expr).eval()?;
                Ok(match token_type {
                    TokenType::Plus => Value::add(left_value, right_value)?,
                    TokenType::Minus => Value::sub(left_value, right_value)?,
                    TokenType::Star => Value::mul(left_value, right_value)?,
                    TokenType::Slash => Value::div(left_value, right_value)?,
                    _ => return Err(ErrorType::InvalidTokenAtPosition { token: *token_type }),
                })
            }
            Self::Unary(token_type, expr) => Ok(match token_type {
                TokenType::Minus => Value::negate(expr.eval()?)?,
                _ => return Err(ErrorType::InvalidTokenAtPosition { token: *token_type }),
            }),
            Self::Var(identifier) => Err(ErrorType::InternalError {
                message: "unimplemented".to_owned(),
            }),
            Self::Func(function, arguments) => Ok(function.call(arguments.eval()?)?),
            Self::Value(value) => Ok(value.clone()),
        }
    }
}
