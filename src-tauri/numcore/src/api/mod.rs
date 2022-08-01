use crate::{context::Context, interpreter, out::*, token, tree, value::Value};

/// Evaluate an expression with the default context.
///
/// See also: `eval_with_static_context` and `eval_with_mutable_context`.
///
/// You cannot declare functions or variables using this function.
///
/// ## Examples
/// This works:
/// ```
/// use numcore::*;
///
/// let result = eval("2 + 2").unwrap();
///
/// assert_eq!(result, Value::from(4));
/// ```
///
///
/// This panics:
/// ```should_panic
/// use numcore::*;
///
/// // Unwraps an error.
/// let result = eval("f(x) = 2x").unwrap();
/// ```
pub fn eval(input: &str) -> EvalResult<Value> {
    eval_with_static_context(input, &Context::default())
}

/// Evaluate an expression not allowing context changes.
///
/// See also `eval`  and`eval_with_mutable_context`.
///
/// As in `eval` you cannot declare functions or variables using this function, but
/// in this case the context can already have its own declarations and use your own settings.
///
/// ## Examples
/// ```
/// use numcore::*;
///
/// // Create context with custom settings
/// let context = Context::new(settings::Rounding::Round(2));
///
/// let res = eval_with_static_context("pi + 1", &context).unwrap();
///
/// assert_eq!(res, Value::from(4.14));
/// ```
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

/// Evaluate an expression or add a declaration to the provided context.
///
/// See also `eval` and `eval_with_static_context`.
///
/// Returns `Ok(Some(Value))` if the request was an evaluation, or `Ok(None)` if it
/// was a declaration.
///
///
/// ## Examples
/// ```
/// use numcore::*;
///
/// let mut context = Context::default();
///
/// // Add a declaration
/// let var_res = eval_with_mutable_context("a = 4", &mut context).unwrap();
/// let func_res = eval_with_mutable_context("g(x,y) = xsin(y)", &mut context).unwrap();
/// // Both results are None
/// assert_eq!(var_res, None);
/// assert_eq!(func_res, None);
///
/// let res = eval_with_mutable_context("g(1,pi/2) + a", &mut context).unwrap();
///
/// assert_eq!(res, Some(Value::from(5)));
///
/// ```
pub fn eval_with_mutable_context(input: &str, context: &mut Context) -> EvalResult<Option<Value>> {
    let input = String::from(input);
    let stream = token::build_stream(input, &context)?;
    let tree = tree::build_tree(stream)?;
    let request = interpreter::interpret_tree(&tree)?;

    println!("REQUEST: {:?}", request);
    request.execute(context)
}
