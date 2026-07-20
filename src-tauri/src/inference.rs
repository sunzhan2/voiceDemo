use sherpa_onnx::{OfflineRecognizer, OfflineRecognizerConfig, OfflineSenseVoiceModelConfig};
use std::path::Path;

pub struct SenseVoiceInference {
    recognizer: Option<OfflineRecognizer>,
    language: String,
    use_itn: bool,
}

impl SenseVoiceInference {
    pub fn new() -> Self {
        Self {
            recognizer: None,
            language: "auto".to_string(),
            use_itn: true,
        }
    }

    pub fn load_model<P: AsRef<Path>>(&mut self, model_dir: P) -> Result<(), String> {
        let model_dir = model_dir.as_ref();

        // Support both model.int8.onnx (default) and model.onnx (fp32)
        let model_path = if model_dir.join("model.int8.onnx").exists() {
            model_dir.join("model.int8.onnx")
        } else {
            model_dir.join("model.onnx")
        };
        let tokens_path = model_dir.join("tokens.txt");

        if !model_path.exists() {
            return Err(format!("模型文件不存在: {}", model_path.display()));
        }
        if !tokens_path.exists() {
            return Err(format!("词表文件不存在: {}", tokens_path.display()));
        }

        let mut config = OfflineRecognizerConfig::default();
        config.model_config.sense_voice = OfflineSenseVoiceModelConfig {
            model: Some(model_path.to_string_lossy().to_string()),
            language: Some(self.language.clone()),
            use_itn: self.use_itn,
        };
        config.model_config.tokens = Some(tokens_path.to_string_lossy().to_string());

        let recognizer = OfflineRecognizer::create(&config)
            .ok_or_else(|| "创建识别器失败".to_string())?;

        self.recognizer = Some(recognizer);
        Ok(())
    }

    pub fn is_loaded(&self) -> bool {
        self.recognizer.is_some()
    }

    pub fn set_language(&mut self, lang: &str) {
        self.language = lang.to_string();
    }

    pub fn set_use_itn(&mut self, itn: bool) {
        self.use_itn = itn;
    }

    /// Transcribe f32 PCM samples at the given sample rate.
    /// sherpa-onnx handles internal resampling to 16kHz.
    pub fn transcribe(&self, sample_rate: u32, audio_data: &[f32]) -> Result<String, String> {
        let recognizer = self
            .recognizer
            .as_ref()
            .ok_or_else(|| "模型未加载".to_string())?;

        if audio_data.is_empty() {
            return Err("音频数据为空".to_string());
        }

        let stream = recognizer.create_stream();
        stream.accept_waveform(sample_rate as i32, audio_data);
        recognizer.decode(&stream);

        let result = stream
            .get_result()
            .ok_or_else(|| "获取识别结果失败".to_string())?;

        Ok(result.text.trim().to_string())
    }
}
