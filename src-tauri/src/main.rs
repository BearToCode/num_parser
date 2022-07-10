#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod parser;
mod term;

use tauri::Manager;

#[tauri::command]
fn generate_graph() {
    println!("I was invoked by JS!");
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window(&"Numerus").unwrap();
            window_shadows::set_shadow(&window, true).expect("Unsupported platform!");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![generate_graph,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
