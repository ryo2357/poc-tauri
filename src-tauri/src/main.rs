#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod handler;
use handler::data_communication::{command_with_error, command_with_object};

mod setup;
use setup::interrupt::{listen_interrupt};


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .setup (|app|{
            listen_interrupt(app);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            command_with_object,
            command_with_error,
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
