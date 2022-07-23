use super::{out::*, token::build_stream, tree::build_tree, value::Value};

pub fn eval(input: &str) -> EvalResult<Value> {
    let input = String::from(input);
    let stream = build_stream(input)?;
    let tree = build_tree(stream)?;
    tree.eval()
}
