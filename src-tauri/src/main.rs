#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio;
mod commands;
mod inference;

use commands::AppState;
use std::sync::Mutex;

/// Returns the path to the models directory.
/// In dev mode: resolves relative to src-tauri/
/// In production: resolves relative to the bundled resources directory.
#[tauri::command]
fn get_model_dir(app: tauri::AppHandle) -> Result<String, String> {
    use tauri::api::path::resource_dir;

    let res_dir = resource_dir(&app.config(), &app.env())
        .ok_or_else(|| "无法获取资源目录".to_string())?;

    let model_dir = res_dir.join("models");
    Ok(model_dir.to_string_lossy().to_string())
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
