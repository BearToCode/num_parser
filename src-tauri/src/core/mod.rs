use numcore;

#[tauri::command]
pub fn create_empty_context() -> numcore::Context {
    numcore::Context::default()
}

#[tauri::command]
pub fn evaluate_with_static_context(
    input: String,
    context: numcore::Context,
) -> Result<String, String> {
    match numcore::eval_with_static_context(&input, &context) {
        Ok(value) => Ok(value.to_string()),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub fn evaluate_with_mutable_context(
    input: String,
    mut context: numcore::Context,
) -> Result<(String, numcore::Context), String> {
    match numcore::eval_with_mutable_context(&input, &mut context) {
        Ok(value) => Ok(match value {
            Some(data) => (data.to_string(), context),
            None => ("()".to_owned(), context),
        }),
        Err(err) => Err(err.to_string()),
    }
}
