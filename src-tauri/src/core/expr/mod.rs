use super::{function::Function, token::Token, value::Value};

pub struct Identifier(String);

pub enum Expression {
    Binary(Box<Expression>, Token, Box<Expression>),
    Unary(Token, Box<Expression>),
    Var(Identifier),
    Func(Identifier, Function),
    Value(Value),
    Equation(Box<Expression>, Box<Expression>),
}
