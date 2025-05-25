
use tauri::{Manager};

#[tauri::command]
async fn greet(app: tauri::AppHandle, script: String) -> Result<(), String> {
    let win = app.get_webview_window("discordMain").unwrap();
    win
        .eval(&script)
        .map_err(|e| format!("Failed to evaluate script: {}", e))?;
    Ok(())
}

#[tauri::command]
fn exit_app() {
  std::process::exit(0x0);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![exit_app])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
