use tauri::{Manager, PhysicalPosition};
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_media::init())
        .invoke_handler(tauri::generate_handler![greet])
        
        .setup(|app|  {

            let window = app.get_webview_window("main").unwrap();
             if let Some(monitor) = window.current_monitor().unwrap() {
                let screen_size = monitor.size();
                let window_size = window.outer_size().unwrap();
                
                let padding = 20;
                let x = screen_size.width as i32 - window_size.width as i32 - padding;
                let y = padding;

                // Position setzen und das Fenster erst JETZT einblenden
                window.set_position(PhysicalPosition::new(x, y)).unwrap();
                window.show().unwrap();
             }
             Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
