mod display;

use crate::{
    context::Context,
    function::builtin,
    out::{ErrorType, EvalResult},
    settings,
    token::{
        self,
        tokentype::{IdentifierType, TokenType},
    },
    value::Value,
};

#[derive(Debug)]
pub enum Request {
    VarDeclaration(String, Box<Expression>),
    FuncDeclaration(String, Vec<String>, Box<Expression>),
    Evaluation(Box<Expression>),
}

impl Request {
    pub fn execute(&self, context: &mut Context) -> EvalResult<Option<Value>> {
        match self {
            Self::Evaluation(expr) => {
                Ok(Some(expr.eval(context, None, 0)?.round(context.rounding)))
            }
            Self::FuncDeclaration(identifier, params, body) => {
                if builtin::reserved_keywords().contains(&&identifier[..]) {
                    Err(ErrorType::ReservedVarName {
                        var_name: identifier.clone(),
                    })
                } else {
                    context.add_function(identifier.clone(), params.clone(), body.clone());
                    Ok(None)
                }
            }
            Self::VarDeclaration(identifier, expression) => {
                if builtin::reserved_keywords().contains(&&identifier[..]) {
                    Err(ErrorType::ReservedFunctionName {
                        func_name: identifier.clone(),
                    })
                } else {
                    context.add_variable(identifier.clone(), expression.clone());
                    Ok(None)
                }
            }
        }
    }
}

/// Every expression variant.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Expression {
    /// A binary operation between two expression.
    Binary(Box<Expression>, TokenType, Box<Expression>),
    /// An unary operation to an expression.
    Unary(TokenType, Box<Expression>),
    /// A variable.
    Var(String),
    /// A function call and its parameters.
    Func(String, Vec<Box<Expression>>),
    /// A literal value.
    Literal(Value),
    /// A union of values.
    Union(Vec<Box<Expression>>),
}

impl Expression {
    pub fn eval(
        &self,
        context: &Context,
        scope: Option<&Context>,
        depth: u32,
    ) -> EvalResult<Value> {
        let depth = depth + 1;

        // Check depth limit
        match context.depth_limit {
            settings::DepthLimit::Limit(max) => {
                if depth >= max {
                    return Err(ErrorType::RecursionDepthLimitReached { limit: max });
                }
            }
            settings::DepthLimit::NoLimit => (),
        }

        match self {
            Self::Binary(left_expr, token_type, right_expr) => {
                let left_value = (**left_expr).eval(context, scope, depth)?;
                let right_value = (**right_expr).eval(context, scope, depth)?;
                Ok(match token_type {
                    // Sum
                    TokenType::Plus => Value::add(left_value, right_value)?,
                    // Subtraction
                    TokenType::Minus => Value::sub(left_value, right_value)?,
                    // Multiplication
                    TokenType::Star => Value::mul(left_value, right_value)?,
                    // Division
                    TokenType::Slash => Value::div(left_value, right_value)?,
                    // Exponentiation
                    TokenType::Caret => Value::exponentiation(left_value, right_value)?,
                    // Modulo
                    TokenType::Percentage => Value::modulo(left_value, right_value)?,
                    // Less than
                    TokenType::LessThan => Value::less_than(left_value, right_value)?,
                    // Greater than
                    TokenType::GreaterThan => Value::greater_than(left_value, right_value)?,
                    // Less or equal to
                    TokenType::LessOrEqualTo => Value::less_or_equal_to(left_value, right_value)?,
                    // Greater or equal to
                    TokenType::GreaterOrEqualTo => {
                        Value::greater_or_equal_to(left_value, right_value)?
                    }
                    // Logical AND
                    TokenType::DoubleAnd => Value::logical_and(left_value, right_value)?,
                    // Logical OR
                    TokenType::DoubleOr => Value::logical_or(left_value, right_value)?,
                    // Equal to
                    TokenType::DoubleEqual => Value::equal_to(left_value, right_value)?,
                    // Not equal to
                    TokenType::NotEqual => Value::not_equal_to(left_value, right_value)?,

                    _ => return Err(ErrorType::InvalidTokenPosition { token: *token_type }),
                })
            }
            Self::Unary(token_type, expr) => Ok(match token_type {
                // Negate
                TokenType::Minus => Value::negate(expr.eval(context, scope, depth)?)?,
                // Not
                TokenType::Exclamation => Value::not(expr.eval(context, scope, depth)?)?,
                _ => return Err(ErrorType::InvalidTokenPosition { token: *token_type }),
            }),
            Self::Union(expressions) => {
                let mut vec = vec![];
                for expr in expressions {
                    vec.push(expr.eval(context, scope, depth)?);
                }
                if vec.len() == 1 {
                    Ok(vec[0].clone())
                } else {
                    Ok(Value::Vector(vec))
                }
            }
            Self::Var(identifier) => {
                // Check built-in vars
                if let Some(var) = builtin::get_built_in_const(identifier) {
                    return Ok(var);
                }

                // Check scope vars
                if let Some(c) = scope {
                    if let Some(expr) = c.get_var(identifier) {
                        return Ok(expr.eval(context, scope, depth)?);
                    }
                }

                // Check context
                if let Some(expr) = context.get_var(identifier) {
                    return Ok(expr.eval(context, scope, depth)?);
                }

                // Try to split the identifier, as it might have not been interpreted correctly
                // in a function declaration, where function parameters were not know at the
                // time of "tokenization".
                let mut joined_context = context.clone();
                // Create a new context with all the data.
                joined_context.join_with(context);
                if let Some(c) = scope {
                    joined_context.join_with(&c);
                }

                let identifiers =
                    token::split_into_identifiers(identifier.clone(), &joined_context);
                let mut product = Value::Float(1.0);
                let mut valid = true;
                let mut argument = Option::None;
                // Iterate over results
                for (i, i_type) in identifiers {
                    match i_type {
                        IdentifierType::Unknown => {
                            // Invalidate result if it still unknown
                            valid = false;
                            break;
                        }
                        IdentifierType::Function => {
                            // use the following identifier as argument
                            // if this is the last identifier, return an error
                            argument = Option::Some(i);
                        }
                        IdentifierType::Var => {
                            if let Some(func_ident) = argument {
                                product = Value::mul(
                                    product,
                                    Self::Func(func_ident, vec![Box::new(Self::Var(i))])
                                        .eval(context, scope, depth)?,
                                )?;
                                argument = Option::None;
                            } else {
                                product =
                                    Value::mul(product, Self::Var(i).eval(context, scope, depth)?)?;
                            }
                        }
                    }
                }

                if valid && argument == Option::None {
                    Ok(product)
                } else {
                    Err(ErrorType::UnknownVar {
                        var_name: identifier.clone(),
                    })
                }
            }
            Self::Func(identifier, arguments) => {
                // Check built-in functions
                if let Some(func) = builtin::get_built_in_function(identifier) {
                    return Ok(func.call(arguments, context, scope, depth)?);
                }
                // Check user-defined ones
                if let Some((names, body)) = context.get_function(&identifier) {
                    let mut inner_scope = {
                        let mut cont = context.clone();
                        // Retrieve the parameters values
                        let params = match value_to_params(
                            names,
                            &Expression::Union(arguments.clone()).eval(context, scope, depth)?,
                        ) {
                            Ok(value) => value,
                            Err(err) => {
                                return match err {
                                    ErrorType::WrongFunctionArgumentsAmount {
                                        func_name: _,
                                        expected,
                                        given,
                                    } => Err(ErrorType::WrongFunctionArgumentsAmount {
                                        func_name: identifier.clone(),
                                        expected,
                                        given,
                                    }),
                                    other => Err(other),
                                }
                            }
                        };

                        for (name, val) in params {
                            cont.add_variable(name, Box::new(Expression::Literal(val)))
                        }

                        cont
                    };

                    match scope {
                        Some(cont) => inner_scope.join_with(cont),
                        None => (),
                    };

                    return Ok(body.eval(context, Some(&inner_scope), depth)?);
                }

                // Try to split the identifier, as it might have not been interpreted correctly
                // in a function declaration, where function parameters were not know at the
                // time of "tokenization".
                let mut joined_context = context.clone();
                // Create a new context with all the data.
                joined_context.join_with(context);
                if let Some(c) = scope {
                    joined_context.join_with(&c);
                }

                let identifiers =
                    token::split_into_identifiers(identifier.clone(), &joined_context);
                let mut product = Value::Float(1.0);
                let mut valid = true;
                let mut argument = Option::None;

                let mut last_i = identifier.clone();
                let mut last_i_type = IdentifierType::Unknown;
                // Iterate over results
                for (i, i_type) in identifiers {
                    match i_type {
                        IdentifierType::Unknown => {
                            // Invalidate result if it still unknown
                            valid = false;
                            break;
                        }
                        IdentifierType::Function => {
                            // use the following identifier as argument
                            // if this is the last identifier, return an error
                            argument = Option::Some(i.clone());
                        }
                        IdentifierType::Var => {
                            if let Some(func_ident) = argument {
                                product = Value::mul(
                                    product,
                                    Self::Func(func_ident, vec![Box::new(Self::Var(i.clone()))])
                                        .eval(context, scope, depth)?,
                                )?;
                                argument = Option::None;
                            } else {
                                product = Value::mul(
                                    product,
                                    Self::Var(i.clone()).eval(context, scope, depth)?,
                                )?;
                            }
                        }
                    }
                    (last_i, last_i_type) = (i.clone(), i_type);
                }

                // If there are no unknown token and last token is a function multiply
                // the product of all the previous vars/functions and the result of the
                // current one.
                if valid && last_i_type == IdentifierType::Function {
                    Value::mul(
                        product,
                        Self::Func(last_i, arguments.clone()).eval(context, scope, depth)?,
                    )
                } else {
                    Err(ErrorType::UnknownFunction {
                        func_name: identifier.clone(),
                    })
                }
            }
            Self::Literal(value) => Ok(value.clone()),
        }
    }
}

fn value_to_params(names: Vec<String>, value: &Value) -> EvalResult<Vec<(String, Value)>> {
    match value {
        Value::Vector(vec) => {
            if names.len() != vec.len() {
                Err(ErrorType::WrongFunctionArgumentsAmount {
                    func_name: "".to_owned(),
                    expected: names.len() as u8,
                    given: vec.len() as u8,
                })
            } else {
                let mut zipped = std::iter::zip(names, vec);
                let mut out = vec![];
                while let Some((name, val)) = zipped.next() {
                    out.push((name, val.clone()));
                }
                Ok(out)
            }
        }
        other => {
            if names.len() != 1 {
                Err(ErrorType::WrongFunctionArgumentsAmount {
                    func_name: "".to_owned(),
                    expected: names.len() as u8,
                    given: 1,
                })
            } else {
                Ok(vec![(names[0].clone(), other.clone())])
            }
        }
    }
}
