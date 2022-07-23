// mod expr;
// mod function;
// mod operators;
// mod out;
// mod token;
// mod tree;
// mod value;

// OLD TO BE REMOVED
use evalexpr;

#[tauri::command]
pub fn evaluate_with_static_context(
    input: String,
    context: evalexpr::HashMapContext,
) -> Result<String, String> {
    match evalexpr::eval_with_context(&input, &context) {
        Ok(value) => Ok(value.to_string()),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub fn evaluate_with_mutable_context(
    input: String,
    mut context: evalexpr::HashMapContext,
) -> Result<(String, evalexpr::HashMapContext), String> {
    match evalexpr::eval_with_context_mut(&input, &mut context) {
        Ok(value) => Ok((value.to_string(), context)),
        Err(err) => Err(err.to_string()),
    }
}
