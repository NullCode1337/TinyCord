use tauri::{Manager};

#[tauri::command]
async fn greet(app: tauri::AppHandle) -> Result<(), String> {
    let cache_dir = app.path().app_cache_dir()
    .expect("Could not resolve cache directory");
    let abs_path = cache_dir.join("browser.js");
    let content = std::fs::read_to_string(abs_path)
    .expect("Should have been able to read the file");
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
    let cache_dir = app_handle.path().app_cache_dir()
        .expect("Could not resolve cache directory");
    std::fs::create_dir_all(&cache_dir)
        .map_err(|e| format!("Failed to create cache directory: {}", e))?;
    let file_path = cache_dir.join("browser.js");
    let response = reqwest::blocking::get(url)
        .map_err(|e| format!("Failed to download script: {}", e))?;
    let content = response.bytes()
        .map_err(|e| format!("Failed to read response bytes: {}", e))?;
    std::fs::write(&file_path, &content)
        .map_err(|e| format!("Failed to write script to cache: {}", e))?;
    Ok(file_path)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            match download(app.handle()) {
                Ok(path) => println!("Script downloaded to: {:?}", path),
                Err(e) => eprintln!("Failed to download script: {}", e),
            }
            
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
