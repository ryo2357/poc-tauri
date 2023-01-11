#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod handler;
use handler::data_communication::{command_with_error, command_with_object};

mod setup;
use setup::interrupt::{listen_interrupt,make_interrupt_thread};
// use setup::interrupt_struct::{make_struct};
// use setup::interrupt_state::{set_state, start_state_process};

use tokio::sync::mpsc;


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {

    let (async_proc_input_tx, async_proc_input_rx) = mpsc::channel(1);
    let (async_proc_output_tx, mut async_proc_output_rx) = mpsc::channel(1);
    
    tauri::Builder::default()
        .setup (|app|{
            listen_interrupt(app);
            make_interrupt_thread(app);
            // make_struct(app);
            // set_state(app);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            command_with_object,
            command_with_error,
            // start_state_process,
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
