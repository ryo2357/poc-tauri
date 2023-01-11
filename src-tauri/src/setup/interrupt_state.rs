
use tauri::App;
// app_handle()メソッドに必要
use tauri::Manager;
use tauri::AppHandle;
use tauri::Wry;
// pub type Wry = Wry<EventLoopMessage>;トレイト内で再公開

use tauri::State;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc,Mutex};
use std::{thread, time};
use serde::{ Serialize, Deserialize };


#[derive(Serialize)]
pub struct InterruptState {
    to_stop: Arc<AtomicBool>,
    app_handle: AppHandle<Wry>,
    count : Arc<Mutex<i32>>,
    is_running: bool,
}

impl InterruptState {
    fn new(app_handle: AppHandle<Wry>) -> Self {
        Self {
            to_stop: Arc::new(AtomicBool::new(false)),
            app_handle: app_handle,
            count: Default::default(),
            is_running: false,
        }
    }

    fn test(&self){
        println!("test-state");
    }

    fn start_process(&mut self)-> Result<String, String> {
        if self.is_running {
            return Err(format!("already running"))
        }
        let to_stop = Arc::clone(&self.to_stop);
        let handle = self.app_handle.clone();
        let count = self.count.clone();
        thread::spawn(move || loop {
            if to_stop.load(Ordering::Relaxed) {
                break;
            }
            println!("worker1 working ");
            thread::sleep(time::Duration::from_secs(1)); // 擬似処理.
            let mut count  = count.lock().unwrap();
            *count += 1;
            handle.emit_all("interrupt-from-back", *count).unwrap();
        });

        self.is_running = true;
        Ok(format!("start process"))
    }
    fn stop_process(&self)-> Result<String, String> {
        Ok(format!("even value "))
    }

}

pub fn set_state(app: &mut App){
    let app_handle = app.app_handle();
    let mut state = InterruptState::new(app_handle);
    app.manage(state);
}


#[tauri::command]
pub fn start_state_process(
    state: State<'_,InterruptState>,
) -> Result<String, String> {
    state.start_process()
}

