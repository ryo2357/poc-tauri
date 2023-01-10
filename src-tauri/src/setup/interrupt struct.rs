
use tauri::App;
use tauri::Manager;

pub struct InterruptStruct {
    app: &mut App,
    thread: Option(std::thread::JoinHandle),
    count : i32,
}

impl InterruptStruct {
    pub fn new(app: &mut App) -> Self {
        Self {
            app: app,
            thread: None,
            count: 0,
        }
    }

    pub fn make_interrupt_thread(&self) {
        if self.thread.is_some() {
            return
        }
        let app_handle = self.app.app_handle();

        self.thread = Some(std::thread::spawn( move|| {
            app_handle
                .emit_all("interrupt-from-struct", self.count.to_string())
                .unwrap();
            std::thread::sleep(std::time::Duration::from_secs(2))
        }));
    }
    pub fn stop_interrupt_thread(&self) {
        if self.thread.is_none() {
            return
        }
        // self.thread.unwrap().join().expect("スレッドの強制終了");
        // 安全に終了するためにはスレッドループ内で参照できる手段で
        // スレッドループ内から終了される必要がある
    
    }
}

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
        std::thread::sleep(std::time::Duration::from_secs(2))

    });
    
}

