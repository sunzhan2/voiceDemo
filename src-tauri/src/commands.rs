use crate::audio::AudioCapture;
use crate::inference::SenseVoiceInference;
use std::sync::Mutex;
use tauri::State;

pub struct AppState {
    pub audio: Mutex<AudioCapture>,
    pub inference: Mutex<SenseVoiceInference>,
}

#[tauri::command]
pub fn start_recording(state: State<AppState>) -> Result<String, String> {
    let mut audio = state.audio.lock().map_err(|e| e.to_string())?;
    audio.start()?;
    Ok("recording".to_string())
}

#[tauri::command]
pub fn stop_recording(state: State<AppState>) -> Result<String, String> {
    let (captured, sample_rate) = {
        let mut audio = state.audio.lock().map_err(|e| e.to_string())?;
        let captured = audio.stop()?;
        let sr = audio.sample_rate();
        (captured, sr)
    };

    if captured.is_empty() {
        return Err("未检测到音频输入".to_string());
    }

    let inference = state.inference.lock().map_err(|e| e.to_string())?;
    let text = inference.transcribe(sample_rate, &captured)?;

    Ok(text)
}

#[tauri::command]
pub fn get_recording_status(state: State<AppState>) -> Result<bool, String> {
    let audio = state.audio.lock().map_err(|e| e.to_string())?;
    Ok(audio.is_recording())
}

#[tauri::command]
pub fn load_model(state: State<AppState>, model_dir: String) -> Result<String, String> {
    let mut inference = state.inference.lock().map_err(|e| e.to_string())?;
    inference.load_model(&model_dir)?;
    Ok("模型加载成功".to_string())
}
