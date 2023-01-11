
use tauri::App;
// app_handle()メソッドに必要
use tauri::Manager;
use tauri::AppHandle;
use tauri::Wry;
// pub type Wry = Wry<EventLoopMessage>;トレイト内で再公開

pub struct InterruptStruct {
    app_handle: AppHandle<Wry>,
    thread: Option<std::thread::JoinHandle<()>>,
    count : i32,
}

impl InterruptStruct {
    pub fn new(app_handle: AppHandle<Wry>) -> Self {
        Self {
            app_handle: app_handle,
            thread: None,
            count: 0,
        }
    }

    pub fn test(&self){
        println!("test");
    }

    pub fn make_interrupt_thread(&self) {
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

pub fn make_struct(app: &mut App){
    let app_handle = app.app_handle();
    let mut my_struct = InterruptStruct::new(app_handle);

    let id = app.listen_global("start-struct-process", move|event| {
        my_struct.test();
    });
    // my_struct.test();
    // 構造体をmoveしてしまうのでこの管理方法は問題がありそう


}



