pub mod builtin;

use crate::{
    out::{ErrorType, EvalResult},
    value::Value,
};

#[derive(Clone, Debug)]
pub struct Function {
    /// The identifier needed to call this function.
    func_identifier: &'static str,
    /// The actual function.
    func: fn(Value) -> EvalResult<Value>,
    /// The function arguments type.
    func_type: FunctionType,
}

impl Function {
    pub fn new(
        func_identifier: &'static str,
        func: fn(Value) -> EvalResult<Value>,
        func_type: FunctionType,
    ) -> Self {
        Self {
            func_identifier,
            func,
            func_type,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum FunctionType {
    // Expects only one argument
    Unary,
    // Expects two arguments
    Binary,
    // Expects any amount
    Vector,
}

impl Function {
    pub fn call(&self, arguments: Value) -> EvalResult<Value> {
        let arguments = match self.func_type {
            FunctionType::Unary => {
                // Avoid vectors
                match arguments {
                    Value::Vector(vector) => {
                        if vector.len() != 1 {
                            return Err(ErrorType::WrongFunctionArgumentsAmount {
                                func_name: String::from(self.func_identifier),
                                expected: 1,
                                given: vector.len() as u8,
                            });
                        } else {
                            vector[0].clone()
                        }
                    }
                    other => other,
                }
            }
            FunctionType::Binary => {
                // Only vectors with two arguments
                match arguments {
                    Value::Vector(ref vector) => {
                        if vector.len() != 2 {
                            return Err(ErrorType::WrongFunctionArgumentsAmount {
                                func_name: String::from(self.func_identifier),
                                expected: 2,
                                given: vector.len() as u8,
                            });
                        } else {
                            arguments
                        }
                    }
                    _ => {
                        return Err(ErrorType::WrongFunctionArgumentsAmount {
                            func_name: String::from(self.func_identifier),
                            expected: 2,
                            given: 1,
                        })
                    }
                }
            }
            FunctionType::Vector => match arguments {
                Value::Vector(_) => arguments,
                other => Value::Vector(vec![other]),
            },
        };

        (self.func)(arguments)
    }
}
