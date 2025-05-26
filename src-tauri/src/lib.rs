use tauri::{Manager};

#[tauri::command]
fn exit_app() {
  std::process::exit(0x0);
}

#[tauri::command]
async fn inject_equicord(app: tauri::AppHandle) -> Result<(), String> {
    let cache_dir = app.path().app_cache_dir().unwrap();
    let abs_path = cache_dir.join("browser.js");
    let content = std::fs::read_to_string(abs_path).unwrap();
    let win = app.get_webview_window("discordMain").unwrap();
    win
        .eval(&content)
        .map_err(|e| format!("Failed to evaluate script: {}", e))?;
    Ok(())
}

pub fn check_equicord(app: &tauri::AppHandle) -> bool {
    let cache_dir = app.path().app_cache_dir().unwrap();
    let path = cache_dir.join("browser.js");
    if !path.exists() { return false; }
    // If it was updated today, no need to update Equicord
    let metadata = std::fs::metadata(path).expect("Failed to find metadata");
    let modified = metadata.modified().expect("Failed to find date modified");
    let modified: chrono::DateTime<chrono::Local> = modified.into();
    let now = chrono::Local::now();
    modified.date_naive() == now.date_naive()
}

fn download_equicord(app: &tauri::AppHandle) -> Result<std::path::PathBuf, String> {
    let url = "https://github.com/Equicord/Equicord/releases/download/latest/browser.js";
    let cache_dir = app.path().app_cache_dir().unwrap();
    let file_path = cache_dir.join("browser.js");
    let response = reqwest::blocking::get(url)
        .map_err(|e| format!("Failed to download link: {}", e))?;
    let content = response.bytes()
        .map_err(|e| format!("Failed to read response bytes: {}", e))?;
    std::fs::write(&file_path, &content).unwrap();
    Ok(file_path)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            if !check_equicord(app.handle()) {
                println!("Equicord out of date, downloading...");
                download_equicord(app.handle()).unwrap();
            }
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            inject_equicord,
            exit_app
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
