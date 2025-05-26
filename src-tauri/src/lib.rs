use tauri::{Manager};

#[tauri::command]
async fn greet(app: tauri::AppHandle) -> Result<(), String> {
    let cache_dir = app.path().app_cache_dir().unwrap();
    let abs_path = cache_dir.join("browser.js");
    let content = std::fs::read_to_string(abs_path).unwrap();
    let win = app.get_webview_window("discordMain").unwrap();
    win
        .eval(&content)
        .map_err(|e| format!("Failed to evaluate script: {}", e))?;
    Ok(())
}

#[tauri::command]
fn exit_app() {
  std::process::exit(0x0);
}

fn download(app_handle: &tauri::AppHandle) -> Result<std::path::PathBuf, String> {
    let url = "https://github.com/Equicord/Equicord/releases/download/latest/browser.js";
    let cache_dir = app_handle.path().app_cache_dir().unwrap();
    std::fs::create_dir_all(&cache_dir).unwrap();
    let file_path = cache_dir.join("browser.js");
    let response = reqwest::blocking::get(url).unwrap();
    let content = response.bytes().unwrap();
    std::fs::write(&file_path, &content).unwrap();
    Ok(file_path)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            download(app.handle()).unwrap();
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            exit_app
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
