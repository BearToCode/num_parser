use crate::{context::Context, interpreter, out::*, token, tree, value::Value};

pub fn eval(input: &str) -> EvalResult<Option<Value>> {
    eval_with_mutable_context(input, &mut Context::new())
}

pub fn eval_with_static_context(input: &str, context: &Context) -> EvalResult<Option<Value>> {
    unimplemented!()
}

pub fn eval_with_mutable_context(input: &str, context: &mut Context) -> EvalResult<Option<Value>> {
    let input = String::from(input);
    let stream = match token::build_stream(input, &context) {
        Ok(value) => value,
        Err(err) => {
            return Err(ErrorType::ErrorDuring {
                operation_name: "parsing",
                error: Box::new(err),
            })
        }
    };
    let tree = match tree::build_tree(stream) {
        Ok(value) => value,
        Err(err) => {
            return Err(ErrorType::ErrorDuring {
                operation_name: "assembling",
                error: Box::new(err),
            })
        }
    };
    let request = match interpreter::interpret_tree(&tree) {
        Ok(value) => value,
        Err(err) => {
            return Err(ErrorType::ErrorDuring {
                operation_name: "interpretation",
                error: Box::new(err),
            })
        }
    };
    println!("REQUEST: {:?}", request);
    match request.execute(context) {
        Ok(value) => Ok(value),
        Err(err) => {
            return Err(ErrorType::ErrorDuring {
                operation_name: "execution",
                error: Box::new(err),
            })
        }
    }
}
