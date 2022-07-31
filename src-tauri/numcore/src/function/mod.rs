pub mod builtin;

use crate::{
    out::{ErrorType, EvalResult},
    value::Value,
};

#[derive(Clone, Debug)]
pub struct Function {
    /// The identifier needed to call this function.
    pub func_identifier: &'static str,
    /// The actual function.
    pub func: fn(Value) -> EvalResult<Value>,
    /// The function arguments type.
    pub args: Arguments,
}

impl Function {
    pub fn new(
        func_identifier: &'static str,
        func: fn(Value) -> EvalResult<Value>,
        func_type: Arguments,
    ) -> Self {
        Self {
            func_identifier,
            func,
            args: func_type,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Arguments {
    /// Expects a constant number of arguments.
    Const(usize),
    /// Expects any amount greater than one.
    Dynamic,
}

impl Function {
    pub fn call(&self, arguments: Value) -> EvalResult<Value> {
        let arguments = match self.args {
            Arguments::Const(count) => {
                if count == 1 {
                    match arguments {
                        Value::Vector(vector) => {
                            if vector.len() != 1 {
                                return Err(ErrorType::WrongFunctionArgumentsAmount {
                                    func_name: self.func_identifier.to_owned(),
                                    expected: 1,
                                    given: vector.len() as u8,
                                });
                            } else {
                                vector[0].clone()
                            }
                        }
                        other => other,
                    }
                } else {
                    match arguments {
                        Value::Vector(vector) => {
                            if vector.len() == count {
                                Value::Vector(vector)
                            } else {
                                return Err(ErrorType::WrongFunctionArgumentsAmount {
                                    func_name: self.func_identifier.to_owned(),
                                    expected: count as u8,
                                    given: vector.len() as u8,
                                });
                            }
                        }
                        _ => {
                            return Err(ErrorType::WrongFunctionArgumentsAmount {
                                func_name: self.func_identifier.to_owned(),
                                expected: count as u8,
                                given: 1,
                            })
                        }
                    }
                }
            }
            Arguments::Dynamic => match arguments {
                Value::Vector(_) => arguments,
                other => Value::Vector(vec![other]),
            },
        };

        (self.func)(arguments)
    }
}
