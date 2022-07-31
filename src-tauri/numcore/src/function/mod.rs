pub mod builtin;

use crate::{
    objects::Expression,
    out::{ErrorType, EvalResult},
    value::{valuetype::ValueType, Value},
    Context,
};

#[derive(Clone)]
pub struct Function {
    /// The identifier needed to call this function.
    pub func_identifier: &'static str,
    /// The actual function.
    pub func: fn(&Vec<Box<Expression>>, &Context) -> EvalResult<Value>,
    /// The function arguments type.
    pub args: Arguments,
}

#[macro_export]
macro_rules! create_func {
    ( $func:ident, $args:expr ) => {
        Function::new(stringify!($func), $func, $args)
    };
}

impl Function {
    pub fn new(
        func_identifier: &'static str,
        func: fn(&Vec<Box<Expression>>, &Context) -> EvalResult<Value>,
        args: Arguments,
    ) -> Self {
        Self {
            func_identifier,
            func,
            args,
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
    pub fn call(
        &self,
        arguments: &Vec<Box<Expression>>,
        context: &Context,
        scope: Option<&Context>,
    ) -> EvalResult<Value> {
        match self.args {
            Arguments::Const(count) => {
                if arguments.len() != count {
                    return Err(ErrorType::WrongFunctionArgumentsAmount {
                        func_name: self.func_identifier.to_owned(),
                        expected: count as u8,
                        given: arguments.len() as u8,
                    });
                }
            }
            _ => (),
        }

        let mut joined_context = Context::new();
        joined_context.join_with(context);
        if let Some(c) = scope {
            joined_context.join_with(c);
        }

        (self.func)(arguments, &joined_context)
    }
}

pub fn type_wrapper<P, T>(
    value: Value,
    target_type: ValueType,
    mut predicate: P,
) -> EvalResult<Value>
where
    P: FnMut(Value) -> EvalResult<T>,
    Value: From<T>,
{
    let original_type = value.get_type();
    let value = value.as_type(&target_type)?;
    Ok(Value::from(predicate(value)?).try_as_type(original_type))
}

pub fn unbox_parameters(arguments: &Vec<Box<Expression>>, context: &Context) -> EvalResult<Value> {
    Expression::Union(arguments.clone()).eval(context, None)
}

#[macro_export]
macro_rules! decl_func {
    ( $identifier:ident, $predicate:expr, $target:expr ) => {
        fn $identifier(arguments: &Vec<Box<Expression>>, context: &Context) -> EvalResult<Value> {
            let unboxed = unbox_parameters(arguments, context)?;
            type_wrapper(unboxed, $target, $predicate)
        }
    };
}
