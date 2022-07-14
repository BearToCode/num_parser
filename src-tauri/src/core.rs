pub mod context;
pub mod functions;
pub mod parser;

#[tauri::command]
pub fn add_declaration() {
    // println!("{}", parser::parse_function(s, functions))
}

#[tauri::command]
pub fn evaluate_expression() {}
