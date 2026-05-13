use tauri::{AppHandle, Manager};
use std::fs;
use std::path::PathBuf;

fn builds_file_path(app: &AppHandle) -> Result<PathBuf, String> {
    app.path()
        .app_data_dir()
        .map(|d| d.join("builds.json"))
        .map_err(|e| format!("Failed to resolve app data dir: {e}"))
}

/// Write the full builds JSON string to disk.
#[tauri::command]
fn save_builds(app: AppHandle, builds_json: String) -> Result<(), String> {
    let path = builds_file_path(&app)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create data dir: {e}"))?;
    }
    fs::write(&path, builds_json)
        .map_err(|e| format!("Failed to write builds file: {e}"))
}

/// Read the builds JSON string from disk. Returns "{}" if the file doesn't exist yet.
#[tauri::command]
fn load_builds(app: AppHandle) -> Result<String, String> {
    let path = builds_file_path(&app)?;
    if path.exists() {
        fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read builds file: {e}"))
    } else {
        Ok("{}".to_string())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![save_builds, load_builds])
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
