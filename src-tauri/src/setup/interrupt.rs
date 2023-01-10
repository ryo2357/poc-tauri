
use tauri::App;
use tauri::Manager;


pub fn listen_interrupt(app: &mut App){
    let id = app.listen_global("interrupt-to-back", |event| {
        println!("got event-name with payload {:?}", event.payload());
    });
}

pub fn make_interrupt_thread(app: &mut App){
    println!("一方通信開始");
    let app_handle = app.app_handle();

    let mut state = false;
    std::thread::spawn(move || loop {

        let message = match state {
            true => {
                state = false;
                "true".to_string()
            },
            false => {
                state = true;
                "false".to_string()
            }
        };

        app_handle
            .emit_all("interrupt-from-back", message)
            .unwrap();
        std::thread::sleep(std::time::Duration::from_secs(5))

    });
    
}

