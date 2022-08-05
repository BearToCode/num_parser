#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
mod core;

use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window(&"Fermi").unwrap();
            window_shadows::set_shadow(&window, true).expect("Unsupported platform!");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            core::evaluate_with_static_context,
            core::evaluate_with_mutable_context,
            core::create_empty_context
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
