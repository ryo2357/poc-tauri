
use tauri::App;
use tauri::Manager;

pub fn listen_interrupt(app: &mut App){
    let id = app.listen_global("front-to-back", |event| {
        println!("got event-name with payload {:?}", event.payload());
    });
}

pub fn make_interrupt_thread(app: &mut App){
    let id = app.listen_global("start-signal", |event| {
        println!("一方通信開始");
        let app_handle = app.app_handle();
        std::thread::spawn(move || loop {
            app_handle
                .emit_all("back-to-front", "ping frontend".to_string())
                .unwrap();
            std::thread::sleep(std::time::Duration::from_secs(5))
        });
    });
    
}

