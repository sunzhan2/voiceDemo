#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio;
mod commands;
mod inference;

use commands::AppState;
use std::sync::Mutex;
use tauri::Manager;

/// Returns the path to the models directory.
/// In dev mode: resolves relative to src-tauri/
/// In production: resolves relative to the bundled resources directory.
#[tauri::command]
fn get_model_dir(app: tauri::AppHandle) -> Result<String, String> {
    let models_dir = app
        .path_resolver()
        .resolve_resource("models")
        .ok_or_else(|| "无法找到资源目录".to_string())?;
    Ok(models_dir.to_string_lossy().to_string())
}

fn main() {
    let app_state = AppState {
        audio: Mutex::new(audio::AudioCapture::new()),
        inference: Mutex::new(inference::SenseVoiceInference::new()),
    };

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            commands::start_recording,
            commands::stop_recording,
            commands::get_recording_status,
            commands::load_model,
            get_model_dir,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
