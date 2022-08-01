use crate::{
    context::{self, Context},
    interpreter,
    out::*,
    token, tree,
    value::Value,
};

pub fn eval(input: &str) -> EvalResult<Option<Value>> {
    eval_with_mutable_context(input, &mut Context::new())
}

pub fn create_empty_context() -> Context {
    context::Context::new()
}

pub fn eval_with_static_context(input: &str, context: &Context) -> EvalResult<Value> {
    let input = String::from(input);
    let stream = token::build_stream(input, &context)?;
    let tree = tree::build_tree(stream)?;
    let request = interpreter::interpret_tree(&tree)?;

    match request {
        crate::objects::Request::Evaluation(_) => (),
        other => return Err(ErrorType::InvalidMutableContext { request: other }),
    }

    let mut context_clone = context.clone();

    match request.execute(&mut context_clone)? {
        Some(result) => Ok(result),
        None => return Err(ErrorType::InvalidMutableContext { request }),
    }
}

pub fn eval_with_mutable_context(input: &str, context: &mut Context) -> EvalResult<Option<Value>> {
    let input = String::from(input);
    let stream = token::build_stream(input, &context)?;
    let tree = tree::build_tree(stream)?;
    let request = interpreter::interpret_tree(&tree)?;

    println!("REQUEST: {:?}", request);
    request.execute(context)
}
