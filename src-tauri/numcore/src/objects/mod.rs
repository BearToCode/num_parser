use crate::{
    context::Context,
    function::builtin,
    out::{ErrorType, EvalResult},
    token::tokentype::TokenType,
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
            Self::Evaluation(expr) => Ok(Some(expr.eval(context, None)?)),
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

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    /// A binary operation between two expression.
    Binary(Box<Expression>, TokenType, Box<Expression>),
    /// An unary operation to an expression.
    Unary(TokenType, Box<Expression>),
    /// A variable.
    Var(String),
    /// A function call and its parameters.
    Func(String, Box<Expression>),
    /// A literal value.
    Literal(Value),
    // TODO: An equation.
    // Equation(Box<Expression>, Box<Expression>),
}

impl Expression {
    pub fn eval(&self, context: &Context, scope: Option<&Context>) -> EvalResult<Value> {
        match self {
            Self::Binary(left_expr, token_type, right_expr) => {
                let left_value = (**left_expr).eval(context, scope)?;
                let right_value = (**right_expr).eval(context, scope)?;
                Ok(match token_type {
                    // Sum
                    TokenType::Plus => Value::add(left_value, right_value)?,
                    // Subtraction
                    TokenType::Minus => Value::sub(left_value, right_value)?,
                    // Multiplication
                    TokenType::Star => Value::mul(left_value, right_value)?,
                    // Division
                    TokenType::Slash => Value::div(left_value, right_value)?,
                    // Aggregation
                    TokenType::Comma => Value::aggregate(left_value, right_value),
                    _ => return Err(ErrorType::InvalidTokenPosition { token: *token_type }),
                })
            }
            Self::Unary(token_type, expr) => Ok(match token_type {
                TokenType::Minus => Value::negate(expr.eval(context, scope)?)?,
                _ => return Err(ErrorType::InvalidTokenPosition { token: *token_type }),
            }),
            Self::Var(identifier) => {
                // Check built-in vars
                if let Some(var) = builtin::get_const(identifier) {
                    return Ok(var);
                }

                if let Some(expr) = context.get_var(identifier) {
                    return Ok(expr.eval(context, scope)?);
                }

                Err(ErrorType::UnknownVar {
                    var_name: identifier.clone(),
                })
            }
            Self::Func(identifier, arguments) => {
                // Check built-in functions
                if let Some(func) = builtin::get_function(identifier) {
                    return Ok(func.call(arguments.eval(context, scope)?)?);
                }
                // Check user-defined ones
                if let Some((names, body)) = context.get_function(&identifier) {
                    let mut inner_scope = {
                        let mut cont = Context::new();
                        let params = match value_to_params(names, &arguments.eval(context, scope)?)
                        {
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
                    return Ok(body.eval(context, Some(&inner_scope))?);
                }

                Err(ErrorType::UnknownFunction {
                    func_name: identifier.clone(),
                })
                // Ok(function.call(arguments.eval()?)?),
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
