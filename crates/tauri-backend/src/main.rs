// tauri-backend minimal
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::Manager;
fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();
            handle.emit_all("app:ready", "unkai backend ready").unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
