use self::{context::Context, functions::Expression};

pub mod context;
pub mod functions;
pub mod parser;

#[tauri::command]
pub fn add_declaration() {
    // println!("{}", parser::parse_function(s, functions))
}

#[tauri::command]
pub fn evaluate_expression(input: String, context: Context) -> Result<Expression, String> {
    println!("input: {}", input);
    let r = parser::parse_function(&input);
    match r {
        Ok(value) => Ok(value),
        Err(err) => {
            println!("{}", err);
            Err(err)
        }
    }
}
