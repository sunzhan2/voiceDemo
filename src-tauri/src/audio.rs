use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Stream, StreamConfig,
};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::sync::Mutex;

pub struct AudioCapture {
    stream: Option<Stream>,
    buffer: Arc<Mutex<Vec<f32>>>,
    is_recording: Arc<AtomicBool>,
    sample_rate: u32,
}

impl AudioCapture {
    pub fn new() -> Self {
        Self {
            stream: None,
            buffer: Arc::new(Mutex::new(Vec::new())),
            is_recording: Arc::new(AtomicBool::new(false)),
            sample_rate: 0,
        }
    }

    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    pub fn start(&mut self) -> Result<(), String> {
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .ok_or_else(|| "未找到麦克风设备".to_string())?;

        let config: StreamConfig = device
            .default_input_config()
            .map_err(|e| format!("无法获取输入配置: {}", e))?
            .into();

        self.sample_rate = config.sample_rate.0;

        let buffer = self.buffer.clone();
        let is_recording = self.is_recording.clone();

        buffer.lock().unwrap().clear();
        is_recording.store(true, Ordering::SeqCst);

        let err_fn = move |err| {
            eprintln!("音频流错误: {}", err);
        };

        let stream = device
            .build_input_stream(
                &config,
                move |data: &[f32], _: &cpal::InputCallbackInfo| {
                    if is_recording.load(Ordering::SeqCst) {
                        if let Ok(mut buf) = buffer.lock() {
                            buf.extend_from_slice(data);
                        }
                    }
                },
                err_fn,
                None,
            )
            .map_err(|e| format!("无法构建音频流: {}", e))?;

        stream
            .play()
            .map_err(|e| format!("无法启动音频流: {}", e))?;

        self.stream = Some(stream);
        Ok(())
    }

    pub fn stop(&mut self) -> Result<Vec<f32>, String> {
        self.is_recording.store(false, Ordering::SeqCst);

        if let Some(stream) = self.stream.take() {
            stream
                .pause()
                .map_err(|e| format!("无法暂停音频流: {}", e))?;
        }

        let captured = self.buffer.lock().unwrap().clone();
        Ok(captured)
    }

    pub fn is_recording(&self) -> bool {
        self.is_recording.load(Ordering::SeqCst)
    }
}

impl Drop for AudioCapture {
    fn drop(&mut self) {
        self.is_recording.store(false, Ordering::SeqCst);
    }
}
