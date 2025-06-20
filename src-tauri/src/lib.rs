use tauri::Manager;

fn get_path(app: &tauri::AppHandle) -> std::path::PathBuf {
    // Browser.js
    #[cfg(target_os = "linux")]
    let cache_dir = app.path().runtime_dir().unwrap();
    #[cfg(target_os = "windows")]
    let cache_dir = app.path().app_cache_dir().unwrap();
    
    cache_dir.join("browser.js")
}

fn check_equicord(app: &tauri::AppHandle) -> bool {
    let path = get_path(app);
    if !path.exists() { return false; }
    // If it was updated today, no need to update Equicord
    let metadata = std::fs::metadata(path).expect("Failed to find metadata");
    let modified = metadata.modified().expect("Failed to find date modified");
    let modified: chrono::DateTime<chrono::Local> = modified.into();
    let now = chrono::Local::now();
    modified.date_naive() == now.date_naive()
}

fn download_equicord(app: &tauri::AppHandle) {
    let path = get_path(app);
    let url = "https://github.com/Equicord/Equicord/releases/download/latest/browser.js";
    let response = reqwest::blocking::get(url)
        .expect("Failed to download link");
    let content = response.bytes()
        .expect("Failed to read response bytes");
    std::fs::write(path, &content).unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            if !check_equicord(app.handle()) {
                println!("Equicord out of date, downloading...");
                download_equicord(app.handle());
            }
            let path = get_path(app.handle());
            let script = std::fs::read_to_string(path).unwrap();

            let window = tauri::WebviewWindowBuilder::new(
                app, "discordMain",
                tauri::WebviewUrl::External("https://discord.com/channels/@me".parse().unwrap())
            )
                .inner_size(1366.0, 768.0)
                .title("TinyCord")
                .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/136.0.0.0 Safari/537.36")
                .initialization_script(script)
                .disable_drag_drop_handler()
                .build()?;

            window.on_window_event(|event| {
                if let tauri::WindowEvent::CloseRequested { .. } = event {
                    std::process::exit(0);
                }
            });

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![ ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
