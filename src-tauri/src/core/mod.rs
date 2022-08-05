use fm_mat::*;

#[tauri::command]
pub fn create_empty_context() -> Context {
    Context::default()
}

#[tauri::command]
pub fn evaluate_with_static_context(input: String, context: Context) -> Result<String, String> {
    match eval_with_static_context(&input, &context) {
        Ok(value) => Ok(value.to_string()),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub fn evaluate_with_mutable_context(
    input: String,
    mut context: Context,
) -> Result<(String, Context), String> {
    use fm_mat::function::*;

    function::builtin::add_built_in_function(create_func!(addone, Arguments::Const(1)));

    decl_func!(
        addone,
        FunctionType::Std,
        |v: Value| Value::add(v, Value::from(1)),
        ValueType::FloatType
    );

    match eval_with_mutable_context(&input, &mut context) {
        Ok(value) => Ok(match value {
            Some(data) => (data.to_string(), context),
            None => ("()".to_owned(), context),
        }),
        Err(err) => Err(err.to_string()),
    }
}
